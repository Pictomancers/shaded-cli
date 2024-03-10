use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
pub struct CollectionConfiguration {
    pub configuration_version: u8,
    pub reshade_version: u8,
    pub name: String,
    pub description: Option<String>,
    pub search_directory: CollectionConfigurationSearchDirectory,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
pub struct CollectionConfigurationSearchDirectory {
    pub path: PathBuf,
    pub max_depth: usize,
}
