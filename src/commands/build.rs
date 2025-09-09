use std::{fs, path::{Path, PathBuf}, process::Command};
use anyhow::{Context, Result, bail};
use super::BuildConfig;


pub fn compile_java_files(java_files: &[PathBuf], build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    println!("Compiling Java files...");

    let mut cmd = Command::new("javac");
    cmd.arg("-d").arg(output_dir);

    // Add classpath if there are dependencies (placeholder for future enhancement)
    if let Some(classpath) = build_classpath() {
        cmd.arg("-cp").arg(classpath);
    }

    // Add all Java files
    for file in java_files {
        cmd.arg(file);
    }

    let output = cmd
        .output()
        .context("Failed to execute javac. Make sure Java is installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Compilation failed:\n{}", stderr);
    }

    println!("Compilation successful!");
    Ok(())
}

pub fn build_classpath() -> Option<String> {
    let lib_dir = Path::new("lib");
    if lib_dir.exists() {
        let mut jars = Vec::new();
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
