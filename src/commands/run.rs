use anyhow::{Context, Result, bail};
use merkle_hash::MerkleTree;
use std::fs;
use std::io::Read;
use std::ops::Not;
use std::process::{Command, ExitStatus};

use crate::commands::build::{build_classpath, compile_sources, discover_source_files};
use crate::commands::load_config;

use super::BuildConfig;

pub fn run_project() -> Result<()> {
    let config = load_config()?;

    let build_config = config.build.unwrap_or_default();
    // if let Ok(mut file) = std::fs::File::open("Cup.lock") {
    //     let tree = MerkleTree::builder("src/")
    //         .algorithm(merkle_hash::Algorithm::Blake3)
    //         .hash_names(false)
    //         .build()?;
    //     let mut buf = vec![];
    //     let _ = file.read_to_end(&mut buf);
    //     dbg!(&buf);
    //     dbg!(&tree.root.item.hash);

    //     if tree.root.item.hash == buf {
    //         if let Some(main_class) = &config.project.main_class {
    //             println!("Running main class: {}", main_class);
    //             run_main_class(main_class, &build_config)?;
    //         } else {
    //             println!("No main class specified in Cup.toml. Compilation complete.");
    //         }
    //     }
    // } else {
    let source_files = discover_source_files(&build_config)?;
    if source_files.is_empty() {
        bail!("No source files found to compile");
    }

    // println!("Found {} source files to compile", source_files.len());

    let output_dir = build_config.output_dir.as_ref().unwrap();
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    // Compile the project
    compile_sources(&source_files, &build_config)?;

    if let Some(main_class) = &config.project.main_class {
        println!("Running main class: {}", main_class);
        run_main_class(main_class, &build_config)?;
    } else {
        println!("No main class specified in Cup.toml. Compilation complete.");
        // }
    }

    Ok(())
}

fn run_main_class(main_class: &str, build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    // Check if this is a Kotlin project by looking for Kotlin files
    let has_kotlin = check_for_kotlin_files(build_config)?;

    let mut cmd = Command::new("java");

    // Build classpath
    let mut classpath_parts = vec![output_dir.to_string()];

    if let Some(lib_classpath) = build_classpath() {
        classpath_parts.push(lib_classpath);
    }

    // For Kotlin projects, we need to ensure Kotlin runtime is available
    if has_kotlin {
        add_kotlin_runtime_to_classpath(&mut classpath_parts);
    }

    let full_classpath = classpath_parts.join(if cfg!(windows) { ";" } else { ":" });
    cmd.arg("-cp").arg(&full_classpath);

    cmd.arg(main_class);

    println!("Executing: java -cp {} {}", full_classpath, main_class);

    let _ = cmd
        .status()
        .inspect(|s| {
            if s.code() == Some(1) {
                let _ = std::fs::remove_file("Cup.lock");
                let _ = run_project();
            }
        })
        .context("Failed to execute java. Make sure Java runtime is installed.")?;

    Ok(())
}

fn check_for_kotlin_files(build_config: &BuildConfig) -> Result<bool> {
    let source_files = discover_source_files(build_config)?;
    Ok(source_files
        .iter()
        .any(|f| f.extension().is_some_and(|ext| ext == "kt")))
}

fn add_kotlin_runtime_to_classpath(classpath_parts: &mut Vec<String>) {
    // common locations of kotlin stdlib
    let kotlin_runtime_paths = [
        "/usr/share/kotlin/lib/kotlin-stdlib.jar",
        "/opt/kotlin/lib/kotlin-stdlib.jar",
        "~/.kotlinc/lib/kotlin-stdlib.jar",
        "/usr/local/share/kotlin/lib/kotlin-stdlib.jar",
        // Windows
        "C:\\Program Files\\Kotlin\\lib\\kotlin-stdlib.jar",
        "C:\\kotlin\\lib\\kotlin-stdlib.jar",
        //mac should be the linux ones idk idc
    ];

    for path in &kotlin_runtime_paths {
        let expanded_path = if path.starts_with('~') {
            if let Some(home) = std::env::var_os("HOME") {
                path.replace('~', &home.to_string_lossy())
            } else {
                path.to_string()
            }
        } else {
            path.to_string()
        };

        if std::path::Path::new(&expanded_path).exists() {
            classpath_parts.push(expanded_path);
            return;
        }
    }

    if let Ok(output) = Command::new("kotlinc").arg("-cp").output()
        && output.status.success()
    {
        let classpath = String::from_utf8_lossy(&output.stdout);
        if !classpath.trim().is_empty() {
            classpath_parts.push(classpath.trim().to_string());
        }
    }
}
