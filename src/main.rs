mod commands;
mod constants;
mod manifests;

use clap::Parser;
use colored::Colorize;
use commands::{manifest::ManifestCommand, package::PackageSubcommand};
use std::process::ExitCode;

#[derive(Debug, Parser)]
enum Commands {
    Package(PackageSubcommand),
    Manifest(ManifestCommand),
}

#[derive(Debug, Parser)]
struct ProgramArgs {
    #[clap(subcommand)]
    cmd: Commands,
}

fn main() -> ExitCode {
    let args = ProgramArgs::parse();

    if let Err(err) = match args.cmd {
        Commands::Package(cmd) => cmd.run(),
        Commands::Manifest(cmd) => cmd.run(),
    } {
        eprintln!("{}: {:?}", "Error".red(), err);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
