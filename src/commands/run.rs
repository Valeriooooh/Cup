use anyhow::{Context, Result, bail};
use std::fs;
use std::process::Command;

use crate::commands::build::compile_java_files;
use crate::commands::{discover_java_files, load_config};

use super::BuildConfig;
use super::build::build_classpath;

pub fn run_project() -> Result<()> {
    let config = load_config()?;
    println!(
        "Building project: {} v{}",
        config.project.name, config.project.version
    );

    let build_config = config.build.unwrap_or_default();
    let java_files = discover_java_files(&build_config)?;

    if java_files.is_empty() {
        bail!("No Java files found to compile");
    }

    println!("Found {} Java files to compile", java_files.len());

    let output_dir = build_config.output_dir.as_ref().unwrap();
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    let _ = compile_java_files(&java_files, &build_config).inspect_err(|e| eprintln!("{}", e));

    if let Some(main_class) = &config.project.main_class {
        println!("Running main class: {}", main_class);
        run_main_class(main_class, &build_config)?;
    } else {
        println!("No main class specified in Cup.toml. Compilation complete.");
    }

    Ok(())
}

fn run_main_class(main_class: &str, build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    let mut cmd = Command::new("java");
    cmd.arg("-cp").arg(output_dir);

    if let Some(classpath) = build_classpath() {
        cmd.arg("-cp").arg(format!("{}:{}", output_dir, classpath));
    }

    cmd.arg(main_class);

    let status = cmd
        .status()
        .context("Failed to execute java. Make sure Java runtime is installed.")?;

    if !status.success() {
        bail!(
            "Program execution failed with exit code: {:?}",
            status.code()
        );
    }

    Ok(())
}
