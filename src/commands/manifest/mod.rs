mod create;
mod format;
mod validate;

use self::{create::CreateCommand, format::FormatCommand};
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum ManifestSubcommands {
    Validate(validate::ValidateCommand),
    Create(CreateCommand),
    Format(FormatCommand),
}

/// Commands for manging and validating shader manifests.
#[derive(Debug, Parser)]
pub struct ManifestCommand {
    #[clap(subcommand)]
    subcommand: ManifestSubcommands,
}

impl ManifestCommand {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            ManifestSubcommands::Validate(cmd) => cmd.run(),
            ManifestSubcommands::Create(cmd) => cmd.run(),
            ManifestSubcommands::Format(cmd) => cmd.run(),
        }
    }
}
