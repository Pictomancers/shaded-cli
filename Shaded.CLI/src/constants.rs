/// The on-disk filename of a shader manifest.
// WARNING: Changing this value is considered a breaking change and will break all existing manifests.
pub const SHADER_MANIFEST_FILENAME: &str = "manifest.json";

/// The on-disk filename of the manifest contained in an archive with a list of included shaders .
// WARNING: Changing this value is considered a breaking change and can break archive parsing in other tools.
pub const SHADERS_MANIFEST_FILENAME: &str = "manifest.json";

/// The on-disk filename of a shaders archive that contains packaged shaders
/// and a shaders.json manifest.
pub const SHADER_ARCHIVE_FILENAME: &str = "shaders.zip";
