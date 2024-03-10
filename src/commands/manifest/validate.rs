use crate::manifests::shader_content::{FileDeclaration, ShaderContentManifest};
use anyhow::{anyhow, Context, Error, Result};
use clap::Parser;
use colored::*;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Run a suite of validation checks against a specific shader manifest.
#[derive(Debug, Parser)]
pub struct ValidateCommand {
    manifest: PathBuf,
}

impl ValidateCommand {
    pub fn run(&self) -> Result<()> {
        let mut invalid = false;
        let manifest_directory = self
            .manifest
            .parent()
            .context("Unable to find manifest parent directory")?;
        let manifest: ShaderContentManifest =
            serde_json::from_str(&fs::read_to_string(&self.manifest)?)?;

        // Informational field validation.
        println!("Validating Metadata");
        {
            let name_invalid = validate_string_entry(&mut invalid, "name", &manifest.name)?;
            let license_invalid = validate_license_path(&mut invalid, &manifest.license_file)?;
            let description_invalid =
                validate_string_entry(&mut invalid, "description", &manifest.description)?;
            let authors_invalid =
                validate_string_entry_vec(&mut invalid, "authors", &manifest.authors)?;

            if !name_invalid && !license_invalid && !description_invalid && !authors_invalid {
                println!("{}", "  * Validated successfully".green());
            }
            println!();
        }

        // File declaration validation.
        if let Some(shaders) = manifest.shaders {
            println!("Validating Shaders");
            let invalid = validate_file_declarations(&mut invalid, &manifest_directory, shaders)?;
            if !invalid {
                println!("{}", "  * Validated successfully".green());
            }
            println!();
        }

        if let Some(textures) = manifest.textures {
            println!("Validating Textures");
            let invalid = validate_file_declarations(&mut invalid, &manifest_directory, textures)?;
            if !invalid {
                println!("{}", "  * Validated successfully".green());
            }
            println!();
        }

        if let Some(presets) = manifest.presets {
            println!("Validating Presets");
            let invalid = validate_file_declarations(&mut invalid, &manifest_directory, presets)?;
            if !invalid {
                println!("{}", "  * Validated successfully".green());
            }
            println!();
        }

        if let Some(addons) = manifest.addons {
            println!("Validating Addons");
            let invalid = validate_file_declarations(&mut invalid, &manifest_directory, addons)?;
            if !invalid {
                println!("{}", "  * Validated successfully".green());
            }
            println!();
        }

        if invalid {
            Err(anyhow!(
                "Manifest was invalid due to one or more validation errors occuring".yellow()
            ))
        } else {
            println!("{}", "Shader manifest is valid".green());
            Ok(())
        }
    }
}

fn validate_string_entry(
    global_invalid: &mut bool,
    field_name: &'static str,
    str: &String,
) -> Result<bool> {
    let mut invalid = false;
    if str != str.trim() {
        invalid = true;
        *global_invalid = true;
        eprintln!(
            "{}",
            format!(
                "  * {} contains empty whitespace at the start or end of entry",
                field_name
            )
            .bright_red()
        );
    }

    Ok(invalid)
}

fn validate_license_path(global_invalid: &mut bool, path: &Option<PathBuf>) -> Result<bool> {
    let mut invalid = false;

    if let Some(path) = path {
        // Rule: License must exist on disk.
        if let Err(err) = std::fs::canonicalize(&path) {
            invalid = true;
            *global_invalid = true;
            eprintln!(
                "{}",
                format!("  * License file was invalid: {:?}", err).bright_red()
            )
        }
    } else {
        eprintln!(
            "{}",
            format!("  * Warning: no license file has been set (this is not an error)")
                .bright_red()
        )
    };
    Ok(invalid)
}

fn validate_string_entry_vec(
    global_invalid: &mut bool,
    field_name: &'static str,
    strings: &Vec<String>,
) -> Result<bool> {
    let mut invalid = false;
    for string in strings {
        if validate_string_entry(global_invalid, field_name, &string)? {
            invalid = true;
            *global_invalid = true;
        }
    }

    Ok(invalid)
}

fn validate_file_declarations(
    global_invalid: &mut bool,
    manifest_directory: &Path,
    declarations: Vec<FileDeclaration>,
) -> Result<bool> {
    let mut invalid = false;

    // Validate Sources.
    let mut sources_with_errors: HashMap<PathBuf, Error> = HashMap::new();
    let mut outputs_with_errors: HashMap<PathBuf, Error> = HashMap::new();
    for declaration in declarations {
        // Source validation
        {
            let source_path: PathBuf = manifest_directory.join(&declaration.source);

            // Rule: Directories cannot start of end with.
            if declaration.source.to_str().unwrap() != declaration.source.to_str().unwrap().trim() {
                sources_with_errors.insert(
                    source_path,
                    anyhow!("contains leading or trailing whitespace"),
                );
                continue;
            }

            // Rule: All files must exist on disk.
            if let Err(err) = std::fs::canonicalize(&source_path) {
                sources_with_errors.insert(source_path, err.into());
                continue;
            }
        }

        // Output validation
        {
            // Rule: Output paths cannot contain directory escapes.
            if declaration
                .output
                .to_str()
                .context("Unable to convert output path to string")?
                .to_string()
                .contains("../")
            {
                outputs_with_errors.insert(
                    declaration.output,
                    anyhow!("Output paths cannot contain directory escapes like '../'"),
                );
                continue;
            }
        }
    }

    if !sources_with_errors.is_empty() {
        invalid = true;
        *global_invalid = true;
        for error in sources_with_errors {
            eprintln!(
                "{}",
                format!("  * Source {:?} was invalid: {}", error.0, error.1).bright_red()
            )
        }
    }

    if !outputs_with_errors.is_empty() {
        invalid = true;
        *global_invalid = true;
        for error in outputs_with_errors {
            eprintln!(
                "{}",
                format!("  * Output {:?} was invalid: {}", error.0, error.1).bright_red()
            )
        }
    }

    Ok(invalid)
}
