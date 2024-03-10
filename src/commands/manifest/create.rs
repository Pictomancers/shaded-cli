use anyhow::Result;
use clap::Parser;

/// Create a new shader manifest.
#[derive(Debug, Parser)]
pub struct CreateCommand {}

impl CreateCommand {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
