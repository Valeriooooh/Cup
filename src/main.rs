use clap::Parser;

use cli::Cli;
use commands::{new::new_project, run::run_project};

mod cli;
mod commands;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::New { project_name } => new_project(project_name, None),
        cli::Commands::Run {} => run_project(),
    }
}
