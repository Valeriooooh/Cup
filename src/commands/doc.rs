use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::builder::OsStr;

use crate::commands::{discover_java_files, load_config};

pub fn create_documentation() -> Result<()> {
    let config = load_config()?;
    println!(
        "Building project: {} v{}",
        config.project.name, config.project.version
    );

    let build_config = config.build.unwrap_or_default();
    let java_files = discover_java_files(&build_config)?;

    if java_files.is_empty() {
        bail!("No Java files to make documentation");
    }

    println!("Found {} Java files to document", java_files.len());

    let doc_dir = build_config.doc_dir.clone().unwrap();
    let mut cmd = Command::new("javadoc");
    for i in java_files {
        if i.extension().is_some_and(|ext| ext == "java") {
            cmd.arg(i);
        }
    }
    cmd.arg("-d").arg(doc_dir);

    let output = cmd
        .output()
        .context("Failed to execute javadoc. Make sure Java is installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Documentation failed:\n{}", stderr);
    }

    println!("Documentation successful!");

    Ok(())
}
