use crate::{
    constants::{SHADERS_MANIFEST_FILENAME, SHADER_ARCHIVE_FILENAME, SHADER_MANIFEST_FILENAME},
    manifests::{
        included_shaders::{IncludedShader, IncludedShadersManifest},
        shader_content::{FileDeclaration, ShaderContentManifest},
    },
};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::{
    fs::{self, create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip_extensions::zip_create_from_directory;

/// Recursively build all shader manifests contained  n the input directory recursively and pack them into
/// a zip archive for use with compatible tools.
#[derive(Debug, Parser)]
pub struct BuildCommand {
    /// Path to the directory that contains one or more manifest files.
    #[arg(short = 'i', long = "input")]
    input_path: PathBuf,

    /// Path to the directory that the 'shaders.picto' archive file will be placed in.
    #[arg(short = 'o', long = "output")]
    output_path: PathBuf,

    /// Delete any existing artifacts generated by another build.
    #[arg(short = 'c', long = "clean")]
    clean: bool,

    /// The depth to recurse when searching for manifest files
    #[arg(short = 'd', long = "max-depth", default_value_t = 1)]
    max_depth: u8,
}

impl BuildCommand {
    pub fn run(&self) -> Result<()> {
        // Prevent running on an existing output path unless specified to clean it.
        if self.output_path.exists() && !self.output_path.read_dir()?.next().is_none() {
            if !self.clean {
                return Err(anyhow!("Output directory already exists. Pass the --clean flag to delete it when running"));
            }
            std::fs::remove_dir_all(&self.output_path)?;
        }

        // Create a working directory that files will be placed in.
        let working_directory = self.output_path.join(".build");
        create_dir_all(&working_directory)?;

        // Recursively find manifests.
        let directories = WalkDir::new(&self.input_path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.metadata()
                    .context("Unable to get DirEntry metadata")
                    .unwrap()
                    .is_dir()
                    && e.path().join(SHADER_MANIFEST_FILENAME).exists()
            });

        // Create an included shaders manifest from the existing manifests.
        let mut included_shader_manifest = IncludedShadersManifest { shaders: vec![] };
        for directory in directories {
            let directory = directory.path();
            let manifest: ShaderContentManifest =
                serde_json::from_str(&read_to_string(directory.join(SHADER_MANIFEST_FILENAME))?)?;
            println!(
                "{} by {:?} for ReShade v{}:",
                manifest.name, manifest.authors, manifest.reshade_version
            );

            // Copy shaders to a build directory
            let shaders = manifest.shaders.unwrap_or_default();
            let shader_path = working_directory.join("Shaders");
            for shader in &shaders {
                println!("[{}] Packing shader: {:?}", manifest.name, shader.source);
                extract_file_declaration(&directory, &shader_path, shader)?;
            }

            let textures = manifest.textures.unwrap_or_default();
            let texture_path = working_directory.join("Textures");
            for texture in &textures {
                println!("[{}] Packing texture: {:?}", manifest.name, texture.source);
                extract_file_declaration(&directory, &texture_path, texture)?;
            }

            let presets = manifest.presets.unwrap_or_default();
            let preset_path = working_directory.join("Presets");
            for preset in &presets {
                println!("[{}] Packing preset: {:?}", manifest.name, preset.source);
                extract_file_declaration(&directory, &preset_path, preset)?;
            }

            let addons = &manifest.addons.unwrap_or_default();
            let addon_path = working_directory.join("Addons");
            for addon in addons {
                println!("[{}] Packing addon: {:?}", manifest.name, addon.source);
                extract_file_declaration(&directory, &addon_path, addon)?;
            }

            if let Some(manifest_license_path) = manifest.license_file.map(|p| directory.join(p)) {
                let license_path = working_directory.join("Licenses");
                println!("[{}] Writing license data", manifest.name);
                fs::create_dir_all(&license_path)?;
                fs::copy(
                    manifest_license_path,
                    license_path.join(format!("LICENSE-{}", manifest.name)),
                )
                .context("Failed to write license")?;
            }

            included_shader_manifest.shaders.push(IncludedShader {
                name: manifest.name,
                authors: manifest.authors,
                description: manifest.description,
                images: manifest.images,
                shader_count: shaders.len(),
                texture_count: textures.len(),
                preset_count: textures.len(),
                addon_count: addons.len(),
            });

            println!();
        }

        println!("Writing shaders.json file with manifest data");
        let included_shader_manifest = serde_json::to_string_pretty(&included_shader_manifest)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&working_directory.join(SHADERS_MANIFEST_FILENAME))?;
        file.write_all(included_shader_manifest.as_bytes())?;
        file.flush()?;

        let zip_path = self.output_path.join(SHADER_ARCHIVE_FILENAME);
        zip_create_from_directory(&zip_path, &working_directory)?;

        Ok(())
    }
}

fn extract_file_declaration(
    source_path: &Path,
    output_path: &Path,
    file: &FileDeclaration,
) -> Result<()> {
    let original_path = source_path.join(&file.source).canonicalize()?;
    let output_path = output_path.join(&file.output);

    fs::create_dir_all(
        &output_path
            .parent()
            .context("Unable to obtain path parent path")?,
    )?;
    fs::copy(original_path, output_path)?;

    Ok(())
}
