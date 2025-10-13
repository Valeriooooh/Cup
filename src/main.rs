use clap::Parser;

use cli::Cli;
use commands::{new::new_project, run::run_project};

use crate::commands::{build::compile_project, doc::create_documentation};

mod cli;
mod commands;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::New {
            project_name,
            kotlin,
        } => new_project(project_name, None, kotlin),
        cli::Commands::Run {} => {
            let _ = run_project().inspect_err(|e| eprintln!("{e}"));
        }
        cli::Commands::Build {} => {
            let _ = compile_project().inspect_err(|e| eprintln!("{e}"));
        }
        cli::Commands::Doc {} => {
            let _ = create_documentation().inspect_err(|e| eprintln!("{e}"));
        }
    }
}
