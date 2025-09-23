use crate::commands::{discover_java_files, load_config};

use super::BuildConfig;
use anyhow::{Context, Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn compile_project() -> Result<()> {
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
    Ok(())
}

pub fn compile_java_files(java_files: &[PathBuf], build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    println!("Compiling Java files...");

    let mut cmd_java = Command::new("javac");
    cmd_java.arg("-d").arg(output_dir);
    // Add classpath if there are dependencies (placeholder for future enhancement)
    if let Some(classpath) = build_classpath() {
        cmd_java.arg("-cp").arg(classpath);
    }

    let mut kt_flag = false;
    let mut cmd_kt = Command::new("kotlinc");
    cmd_kt.arg("-d").arg(output_dir);
    // Add classpath if there are dependencies (placeholder for future enhancement)
    if let Some(classpath) = build_classpath() {
        cmd_kt.arg("-cp").arg(classpath);
    }
    cmd_kt.arg("-include-runtime");

    for file in java_files {
        if let Some(a) = file.extension() {
            match a.to_str() {
                Some("java") => {
                    cmd_java.arg(file);
                }
                Some("kt") => {
                    cmd_kt.arg(file);
                    kt_flag = true;
                }
                None | Some(_) => {}
            }
        }
    }
    let output = cmd_java
        .output()
        .context("Failed to execute javac. Make sure Java is installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Compilation failed:\n{}", stderr);
    }

    if kt_flag {
        let output = cmd_kt
            .output()
            .context("Failed to execute kotlinc. Make sure Java is installed and in PATH.")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Compilation failed:\n{}", stderr);
        }
    }

    println!("Compilation successful!");
    Ok(())
}

pub fn build_classpath() -> Option<String> {
    let lib_dir = Path::new("lib");
    if lib_dir.exists() {
        let mut jars = vec![];
        if let Ok(entries) = fs::read_dir(lib_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "jar") {
                    jars.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
        if !jars.is_empty() {
            println!("{:?}", jars);
            return Some(jars.join(":"));
        }
    }
    None
}
