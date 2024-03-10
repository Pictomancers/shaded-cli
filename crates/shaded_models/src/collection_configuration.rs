use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
pub struct CollectionConfiguration {
    pub configuration_version: u8,
    pub reshade_version: u8,
    pub name: String,
    pub description: Option<String>,
    pub search_directory: CollectionShaderpackPath,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
pub struct CollectionShaderpackPath {
    pub path: PathBuf,
    pub max_depth: usize,
}
