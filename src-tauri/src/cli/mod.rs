mod app;
mod common;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    App(app::Args),
}

/// CLI entrypoint to run.
pub async fn execute() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            Commands::App(args) => app::execute(&cli, args).await,
        },
        // Default command
        None => app::execute(&cli, &app::Args::default()).await,
    }
}
