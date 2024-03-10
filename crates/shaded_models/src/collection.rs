// References:
//  - https://github.com/Pictomancers/shaded-schemas/tree/main/collection

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct CollectionManifest {
    pub manifest_version: u8,
    #[serde(rename(serialize = "ReShadeVersion", deserialize = "ReShadeVersion"))]
    pub reshade_version: u8,
    pub name: String,
    pub description: Option<String>,
    pub shader_packs: Vec<CollectionShaderPack>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct CollectionShaderPack {
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub images: Option<Vec<String>>,
    pub shader_count: usize,
    pub texture_count: usize,
    pub preset_count: usize,
    pub addon_count: usize,
}

/// Constant values relating to `Shaded`` collections.
pub mod constants {
    // WARNING: Changing any value here iss considered a breaking change and WILL break other tools.
    // please take considerable care when if/when doing so.

    /// The on-disk filename of a collection archive that contains shaderpacks and
    /// a collection manifest file at the root of the archive.
    pub const COLLECTION_ARCHIVE_FILENAME: &str = "collection.zip";

    /// The on-disk filename of the manifest contained in a collection with metadata about the collection.
    pub const COLLECTION_MANIFEST_FILENAME: &str = "collection.json";

    /// The name of the shader directory relative to the collection folder root.
    pub const SHADER_DIRECTORY_NAME: &str = "Shaders";

    /// The name of the texture directory relative to the collection folder root.
    pub const TEXTURE_DIRECTORY_NAME: &str = "Textures";

    /// The name of the preset directory relative to the collection folder root.
    pub const PRESET_DIRECTORY_NAME: &str = "Presets";

    /// The name of the addon directory relative to the collection folder root.
    pub const ADDON_DIRECTORY_NAME: &str = "Addons";

    /// The name of the licenses directory relative to the collection folder root.
    pub const LICENSE_DIRECTORY_NAME: &str = "Licenses";
}
