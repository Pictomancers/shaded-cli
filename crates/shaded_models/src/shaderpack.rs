// References:
//  - https://github.com/Pictomancers/shaded-schemas/tree/main/shaderpack

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct ShaderPackManifest {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct FileDeclaration {
    pub source: PathBuf,
    pub output: PathBuf,
}

#[derive(Error, Debug)]
pub enum FileDeclarationCopyErrorKind {
    #[error("unable to obtain base path from file output declaration")]
    BasePathNotFound,

    #[error("Failed to canonicalize source file")]
    SourceFileCanonicalizationFailure(std::io::Error),

    #[error("failed to create the leading directories for the output path")]
    OutputDirectoryCreateFailure(std::io::Error),

    #[error("failed to create the copy source file to output path")]
    OutputFileCopyFailure(std::io::Error),
}

impl FileDeclaration {
    /// Copy this file declaration to the given output directory and create all missing sub-directories in [`FileDeclaration::output`] while doing so.
    ///
    /// # Arguments
    /// * `input_base_path`: The base input directory of declaration to be used when converting relative [`FileDeclaration::source`] paths into absolute paths.
    /// * `output_base_path`: The base output directory that the [`FileDeclaration::output`] path will be appended to.
    pub fn copy_to_output_path(
        &self,
        input_base_path: &Path,
        output_base_path: &Path,
    ) -> Result<(), FileDeclarationCopyErrorKind> {
        let original_path = input_base_path
            .join(&self.source)
            .canonicalize()
            .map_err(|err| FileDeclarationCopyErrorKind::SourceFileCanonicalizationFailure(err))?;
        let output_path = output_base_path.join(&self.output);

        let Some(output_path_parent) = output_path.parent() else {
            return Err(FileDeclarationCopyErrorKind::BasePathNotFound);
        };

        fs::create_dir_all(output_path_parent)
            .map_err(|err| FileDeclarationCopyErrorKind::OutputDirectoryCreateFailure(err))?;
        fs::copy(original_path, output_path)
            .map_err(|err| FileDeclarationCopyErrorKind::OutputFileCopyFailure(err))?;

        Ok(())
    }
}

/// Constant values relating to `Shaded` Shaderpacks.
pub mod constants {
    // WARNING: Changing any value here iss considered a breaking change and WILL break other tools.
    // please take considerable care when if/when doing so.

    /// The on-disk filename of a shaderpack manifest.
    pub const SHADERPACK_MANIFEST_FILENAME: &str = "shaded-manifest.json";
}
