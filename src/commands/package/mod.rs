pub mod build;

use self::build::BuildCommand;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum PackageSubcommands {
    Build(BuildCommand),
}

/// Commands for managing shader packages.
#[derive(Debug, Parser)]
pub struct PackageSubcommand {
    #[clap(subcommand)]
    subcommand: PackageSubcommands,
}

impl PackageSubcommand {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            PackageSubcommands::Build(cmd) => cmd.run(),
        }
    }
}
