use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct ShaderContentManifest {
    pub manifest_version: u8,
    #[serde(rename(serialize = "ReShadeVersion", deserialize = "ReShadeVersion"))]
    pub reshade_version: u8,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub license_file: Option<PathBuf>,
    pub images: Option<Vec<String>>,
    pub shaders: Option<Vec<FileDeclaration>>,
    pub textures: Option<Vec<FileDeclaration>>,
    pub presets: Option<Vec<FileDeclaration>>,
    pub addons: Option<Vec<FileDeclaration>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct FileDeclaration {
    pub source: PathBuf,
    pub output: PathBuf,
}
