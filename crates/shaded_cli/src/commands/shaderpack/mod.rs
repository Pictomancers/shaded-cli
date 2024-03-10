mod validate;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum ManifestSubcommands {
    Validate(validate::ValidateCommand),
}

/// Commands for manging and validating shader manifests.
#[derive(Debug, Parser)]
pub struct ShaderpackCommandBase {
    #[clap(subcommand)]
    subcommand: ManifestSubcommands,
}

impl ShaderpackCommandBase {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            ManifestSubcommands::Validate(cmd) => cmd.run(),
        }
    }
}
