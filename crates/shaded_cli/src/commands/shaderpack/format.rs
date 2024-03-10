use anyhow::Result;
use clap::Parser;
use shaded_models::shaderpack::ShaderPackManifest;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

/// Format a shaderpack manifest file.
#[derive(Debug, Parser)]
pub struct FormatCommand {
    /// Path to the shaderpack manifest file.
    pub manifest_path: PathBuf,
}

impl FormatCommand {
    pub fn run(&self) -> Result<()> {
        let mut manifest: ShaderPackManifest =
            serde_json::from_str(&fs::read_to_string(&self.manifest_path)?)?;
        manifest.shaders.as_deref_mut().map(|s| s.sort());
        manifest.textures.as_deref_mut().unwrap_or_default().sort();
        manifest.presets.as_deref_mut().unwrap_or_default().sort();
        manifest.addons.as_deref_mut().unwrap_or_default().sort();
        let manifest = serde_json::to_string_pretty(&manifest)?;

        // Overwrite the file with the new contents.
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(&self.manifest_path)?;
        file.write_all(manifest.as_bytes())?;
        file.flush()?;

        Ok(())
    }
}
