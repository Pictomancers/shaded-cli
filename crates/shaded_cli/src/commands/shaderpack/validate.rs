use anyhow::{anyhow, Context, Error, Result};
use clap::Parser;
use colored::*;
use shaded_models::shaderpack::{FileDeclaration, ShaderPackManifest};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum ProblemType {
    Warning(String),
    Error(Error),
}

/// Run validation checks on a shaderpack manifest.
#[derive(Debug, Parser)]
pub struct ValidateCommand {
    /// Path to the shaderpack manifest file.
    manifest_path: PathBuf,
}

impl ValidateCommand {
    pub fn run(&self) -> Result<()> {
        let mut validation_failure = false;
        let mut validation_warning = false;

        let manifest_directory = self
            .manifest_path
            .parent()
            .context("Unable to find manifest parent directory")?;

        println!("Loading Shaderpack File");
        let manifest: ShaderPackManifest = serde_json::from_str(
            &fs::read_to_string(&self.manifest_path)
                .context("An error occured while reading shaderpack manifest")?,
        )
        .context("An error ocucred while parsing shaderpack manifest")?;

        // Informational field validation.
        {
            println!("Validating Information");
            let mut info_validation_problems: HashMap<String, ProblemType> = HashMap::new();

            // Validate name.
            if let Some(name_problems) = validate_string_entry("name", &manifest.name) {
                info_validation_problems.extend(name_problems);
            }

            // Validate license.
            if let Some(license_problems) = validate_license_entry(
                "license",
                &manifest.license_file.map(|f| manifest_directory.join(f)),
            ) {
                info_validation_problems.extend(license_problems);
            }

            // Validate description.
            if let Some(description_problems) =
                validate_string_entry("description", &manifest.description)
            {
                info_validation_problems.extend(description_problems);
            }

            // Validate authors.
            if let Some(authors_problems) = validate_string_entry_vec("authors", &manifest.authors)
            {
                info_validation_problems.extend(authors_problems);
            }

            if info_validation_problems.is_empty() {
                println!("{}", "  * Validated successfully".green());
            } else {
                for problem in info_validation_problems {
                    match problem.1 {
                        ProblemType::Warning(warn) => {
                            validation_warning = true;
                            eprintln!(
                                "{}",
                                format!("  * Warning with {}: {:?}", problem.0, warn).yellow()
                            );
                        }
                        ProblemType::Error(err) => {
                            validation_failure = true;
                            eprintln!(
                                "{}",
                                format!("  * Error with {}: {:?}", problem.0, err).red()
                            );
                        }
                    }
                }
            }
            println!();
        }

        // Shader validation.
        if let Some(shaders) = manifest.shaders {
            println!("Validating Shaders");
            let mut info_validation_problems: HashMap<String, ProblemType> = HashMap::new();

            if let Some(file_declaration_problems) =
                validate_file_declarations(&manifest_directory, shaders)?
            {
                info_validation_problems.extend(file_declaration_problems);
            }

            if info_validation_problems.is_empty() {
                println!("{}", "  * Validated successfully".green());
            } else {
                for problem in info_validation_problems {
                    match problem.1 {
                        ProblemType::Warning(warn) => {
                            validation_warning = true;
                            eprintln!(
                                "{}",
                                format!("  * Warning with {}: {:?}", problem.0, warn).yellow()
                            );
                        }
                        ProblemType::Error(err) => {
                            validation_failure = true;

                            eprintln!(
                                "{}",
                                format!("  * Error with {}: {:?}", problem.0, err).red()
                            );
                        }
                    }
                }
            }
            println!();
        }

        if let Some(textures) = manifest.textures {
            println!("Validating Textures");
            let mut info_validation_problems: HashMap<String, ProblemType> = HashMap::new();

            if let Some(file_declaration_problems) =
                validate_file_declarations(&manifest_directory, textures)?
            {
                info_validation_problems.extend(file_declaration_problems);
            }

            if info_validation_problems.is_empty() {
                println!("{}", "  * Validated successfully".green());
            } else {
                for problem in info_validation_problems {
                    match problem.1 {
                        ProblemType::Warning(warn) => {
                            validation_warning = true;
                            eprintln!(
                                "{}",
                                format!("  * Warning with {}: {:?}", problem.0, warn).yellow()
                            );
                        }
                        ProblemType::Error(err) => {
                            validation_failure = true;

                            eprintln!(
                                "{}",
                                format!("  * Error with {}: {:?}", problem.0, err).red()
                            );
                        }
                    }
                }
            }
            println!();
        }

        if let Some(presets) = manifest.presets {
            println!("Validating Presets");
            let mut info_validation_problems: HashMap<String, ProblemType> = HashMap::new();

            if let Some(file_declaration_problems) =
                validate_file_declarations(&manifest_directory, presets)?
            {
                info_validation_problems.extend(file_declaration_problems);
            }

            if info_validation_problems.is_empty() {
                println!("{}", "  * Validated successfully".green());
            } else {
                for problem in info_validation_problems {
                    match problem.1 {
                        ProblemType::Warning(warn) => {
                            validation_warning = true;
                            eprintln!(
                                "{}",
                                format!("  * Warning with {}: {:?}", problem.0, warn).yellow()
                            );
                        }
                        ProblemType::Error(err) => {
                            validation_failure = true;

                            eprintln!(
                                "{}",
                                format!("  * Error with {}: {:?}", problem.0, err).red()
                            );
                        }
                    }
                }
            }
            println!();
        }

        if let Some(addons) = manifest.addons {
            println!("Validating Addons");
            let mut info_validation_problems: HashMap<String, ProblemType> = HashMap::new();

            if let Some(file_declaration_problems) =
                validate_file_declarations(&manifest_directory, addons)?
            {
                info_validation_problems.extend(file_declaration_problems);
            }

            if info_validation_problems.is_empty() {
                println!("{}", "  * Validated successfully".green());
            } else {
                for problem in info_validation_problems {
                    match problem.1 {
                        ProblemType::Warning(warn) => {
                            validation_warning = true;
                            eprintln!(
                                "{}",
                                format!("  * Warning with {}: {:?}", problem.0, warn).yellow()
                            );
                        }
                        ProblemType::Error(err) => {
                            validation_failure = true;

                            eprintln!(
                                "{}",
                                format!("  * Error with {}: {:?}", problem.0, err).red()
                            );
                        }
                    }
                }
            }
            println!();
        }

        if validation_failure {
            Err(anyhow!(
                "Manifest was invalid due to one or more validation errors occuring"
                    .yellow()
                    .bold()
            ))
        } else if validation_warning {
            println!(
                "{}",
                "Shader manifest is valid, but has validation warnings".yellow()
            );
            Ok(())
        } else {
            println!("{}", "Shader manifest is valid".green());
            Ok(())
        }
    }
}

// String

fn validate_string_entry(
    field_name: &'static str,
    str: &String,
) -> Option<HashMap<String, ProblemType>> {
    let mut failures = HashMap::new();

    if str != str.trim() {
        failures.insert(
            field_name.to_owned(),
            ProblemType::Error(anyhow!(
                "Contains empty whitespace at start or end of entry./"
            )),
        );
    }

    if failures.is_empty() {
        None
    } else {
        Some(failures)
    }
}

fn validate_license_entry(
    field_name: &'static str,
    path: &Option<PathBuf>,
) -> Option<HashMap<String, ProblemType>> {
    let mut failures = HashMap::new();

    if let Some(path) = path {
        // Rule(error): License must exist on disk.
        if let Err(err) = std::fs::canonicalize(&path) {
            failures.insert(field_name.to_owned(), ProblemType::Error(err.into()));
        }
    } else {
        // Rule(warning): Shaderpacks should contain a LICENSE file.
        failures.insert(
            field_name.to_owned(),
            ProblemType::Warning("No license file has been set".to_owned()),
        );
    };

    if failures.is_empty() {
        None
    } else {
        Some(failures)
    }
}

fn validate_string_entry_vec(
    field_name: &'static str,
    strings: &Vec<String>,
) -> Option<HashMap<String, ProblemType>> {
    let mut failures = HashMap::default();
    for string in strings {
        if let Some(string_failures) = validate_string_entry(field_name, &string) {
            failures.extend(string_failures);
        }
    }

    if failures.is_empty() {
        None
    } else {
        Some(failures)
    }
}

fn validate_file_declarations(
    manifest_directory: &Path,
    declarations: Vec<FileDeclaration>,
) -> Result<Option<HashMap<String, ProblemType>>> {
    let mut failures = HashMap::default();

    for declaration in declarations {
        // Source validation
        {
            let source_path: PathBuf = manifest_directory.join(&declaration.source);

            // Rule: Directories cannot start of end with.
            if declaration.source.to_str().unwrap() != declaration.source.to_str().unwrap().trim() {
                failures.insert(
                    source_path.to_str().unwrap_or_default().to_owned(),
                    ProblemType::Error(anyhow!("contains leading or trailing whitespace")),
                );
                continue;
            }

            // Rule: All files must exist on disk.
            if let Err(err) = std::fs::canonicalize(&source_path) {
                failures.insert(
                    source_path.to_str().unwrap_or_default().to_owned(),
                    ProblemType::Error(err.into()),
                );
                continue;
            }
        }

        // Output validation
        {
            // Rule: Output paths cannot contain directory escapes
            let output_string = declaration
                .output
                .to_str()
                .context("Unable to convert output path to string")?
                .to_string();
            if output_string.contains("./") {
                failures.insert(
                    declaration.output.to_str().unwrap_or_default().to_owned(),
                    ProblemType::Error(anyhow!(
                        "Output paths cannot contain directory escapes like '../'"
                    )),
                );
                continue;
            }
        }
    }

    if failures.is_empty() {
        Ok(None)
    } else {
        Ok(Some(failures))
    }
}
