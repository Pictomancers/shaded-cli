mod format;
mod validate;

use self::format::FormatCommand;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum ManifestSubcommands {
    Validate(validate::ValidateCommand),
    Format(FormatCommand),
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
            ManifestSubcommands::Format(cmd) => cmd.run(),
        }
    }
}
