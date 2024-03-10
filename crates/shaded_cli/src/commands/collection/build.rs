use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use shaded_models::{
    collection::{
        constants::{
            ADDON_DIRECTORY_NAME, COLLECTION_ARCHIVE_FILENAME, COLLECTION_MANIFEST_FILENAME,
            LICENSE_DIRECTORY_NAME, PRESET_DIRECTORY_NAME, SHADER_DIRECTORY_NAME,
            TEXTURE_DIRECTORY_NAME,
        },
        CollectionManifest, CollectionShaderPack,
    },
    collection_configuration::CollectionConfiguration,
    shaderpack::{constants::SHADERPACK_MANIFEST_FILENAME, ShaderPackManifest},
};
use std::{
    fs::{self, create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};
use walkdir::{DirEntry, WalkDir};
use zip_extensions::zip_create_from_directory;

/// Build a collection using the provided configuration file and output it to the given directory
/// as a zip archive.
#[derive(Debug, Parser)]
pub struct BuildCommand {
    /// Path to the collection configuration file.
    #[arg(short = 'c', long = "configuration")]
    configuration_path: PathBuf,

    /// Path to a directory that the built collection archive should be placed at.
    #[arg(short = 'o', long = "output")]
    output_path: PathBuf,

    /// Delete any existing artifacts generated by another build.
    #[arg(short = 'd', long = "delete-existing")]
    delete_existing: bool,
}

impl BuildCommand {
    pub fn run(&self) -> Result<()> {
        // Prevent overwriting an existing collection build output unless specified to delete it.
        if self.output_path.exists()
            && !self
                .output_path
                .read_dir()
                .context("Failed to read output path directory")?
                .next()
                .is_none()
        {
            if !self.delete_existing {
                return Err(anyhow!("There are already files in the output directory. Pass the --delete-existing flag to delete any existing files."));
            }
            fs::remove_dir_all(&self.output_path).context("Failed to existing output directory")?;
        }

        // Create a build directory to temporarily place all files in before they're turned into an archive.
        let temp_build_directory = self.output_path.join(".build");
        create_dir_all(&temp_build_directory)
            .context("Failed to create a temporary build directory")?;

        // Load the collection config and use its parent directory of it as the search directory base.
        let configuration: CollectionConfiguration =
            toml::from_str(&fs::read_to_string(&self.configuration_path)?)?;

        // Recursively find shaderpack manifests.
        let directories: Vec<DirEntry> = WalkDir::new(
            self.configuration_path
                .parent()
                .context("Failed to get the parent directory of the configuration file")?
                .join(configuration.search_directory.path)
                .canonicalize()
                .context("Failed to canconicalize configuration search directory")?,
        )
        .max_depth(configuration.search_directory.max_depth)
        .into_iter()
        .filter_map(|e| e.ok()) // Only grab DirEntry's that are not errors.
        .filter(|e| {
            e.metadata()
                .map_or(false, |m| m.is_dir()) // DirEntry must be a directory.
                && e.path().join(SHADERPACK_MANIFEST_FILENAME).exists() // DirEntry must have a shader manifest.
        })
        .collect();

        // If no shaderpack manifests are found, return early with an error.
        if directories.len() == 0 {
            bail!("No manifests could be found inside of the search directory or {} directories under it", configuration.search_directory.max_depth);
        }

        // For every directory with a manifest found, read the manifest and pack it into the collection.
        let mut included_shaderpacks = vec![];
        for directory in directories {
            let directory = directory.path();
            let manifest: ShaderPackManifest = serde_json::from_str(&read_to_string(
                directory.join(SHADERPACK_MANIFEST_FILENAME),
            )?)?;
            println!(
                "{} by {:?} for ReShade v{}:",
                manifest.name, manifest.authors, manifest.reshade_version
            );

            // Copy shaders to the build directory.
            let shaders = manifest.shaders.unwrap_or_default();
            let shader_path = temp_build_directory.join(SHADER_DIRECTORY_NAME);
            for shader in &shaders {
                println!("[{}] Packing shader: {:?}", manifest.name, shader.source);
                shader.copy_to_output_path(&directory, &shader_path)?;
            }

            // Copy textures to the build directory.
            let textures = manifest.textures.unwrap_or_default();
            let texture_path = temp_build_directory.join(TEXTURE_DIRECTORY_NAME);
            for texture in &textures {
                println!("[{}] Packing texture: {:?}", manifest.name, texture.source);
                texture.copy_to_output_path(&directory, &texture_path)?;
            }

            // Copy presets to the build directory.
            let presets = manifest.presets.unwrap_or_default();
            let preset_path = temp_build_directory.join(PRESET_DIRECTORY_NAME);
            for preset in &presets {
                println!("[{}] Packing preset: {:?}", manifest.name, preset.source);
                preset.copy_to_output_path(&directory, &preset_path)?;
            }

            // Copy addons to build directory.
            let addons = &manifest.addons.unwrap_or_default();
            let addon_path = temp_build_directory.join(ADDON_DIRECTORY_NAME);
            for addon in addons {
                println!("[{}] Packing addon: {:?}", manifest.name, addon.source);
                addon.copy_to_output_path(&directory, &addon_path)?;
            }

            // Copy licenses to build directory.
            if let Some(manifest_license_path) = manifest.license_file.map(|p| directory.join(p)) {
                let license_path = temp_build_directory.join(LICENSE_DIRECTORY_NAME);
                println!("[{}] Writing license data", manifest.name);
                fs::create_dir_all(&license_path)?;
                fs::copy(
                    manifest_license_path,
                    license_path.join(format!("LICENSE-{}", manifest.name)),
                )
                .context("Failed to write license")?;
            }

            // Add this shaderpack to the list of this collection's includued shaderpacks.
            included_shaderpacks.push(CollectionShaderPack {
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

        // Write a collection.json file to the build directory root.
        println!("Writing collection.json file with collection data");

        let collection = serde_json::to_string_pretty(&CollectionManifest {
            manifest_version: 1,
            name: configuration.name,
            description: configuration.description,
            reshade_version: configuration.reshade_version,
            shader_packs: included_shaderpacks,
        })?;
        let mut collection_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&temp_build_directory.join(COLLECTION_MANIFEST_FILENAME))?;
        collection_file.write_all(collection.as_bytes())?;
        collection_file.flush()?;

        // Zip
        let zip_path = self.output_path.join(COLLECTION_ARCHIVE_FILENAME);
        zip_create_from_directory(&zip_path, &temp_build_directory)?;

        Ok(())
    }
}
