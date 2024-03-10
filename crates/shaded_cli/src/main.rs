mod commands;

use clap::Parser;
use colored::Colorize;
use commands::{collection::CollectionCommandBase, shaderpack::ShaderpackCommandBase};
use std::process::ExitCode;

#[derive(Debug, Parser)]
enum Commands {
    /// Commands relating to Shaded collections.
    Collection(CollectionCommandBase),

    /// Commands relating to Shaded Shaderpacks.
    Shaderpack(ShaderpackCommandBase),
}

#[derive(Debug, Parser)]
struct ProgramArgs {
    #[clap(subcommand)]
    cmd: Commands,
}

fn main() -> ExitCode {
    let args = ProgramArgs::parse();

    if let Err(err) = match args.cmd {
        Commands::Collection(cmd) => cmd.run(),
        Commands::Shaderpack(cmd) => cmd.run(),
    } {
        eprintln!("{}: {:?}", "Error".red(), err);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
