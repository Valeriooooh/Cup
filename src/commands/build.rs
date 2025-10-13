use crate::commands::load_config;

use super::BuildConfig;
use anyhow::{Context, Result, bail};
use dialoguer::console::{Emoji, style};
use indicatif::ProgressStyle;
use merkle_hash::MerkleTree;
use std::{
    convert::Infallible,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

pub fn compile_project() -> Result<()> {
    let config = load_config()?;

    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Resolving packages...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );

    let build_config = config.build.unwrap_or_default();
    let source_files = discover_source_files(&build_config)?;

    if source_files.is_empty() {
        bail!("No source files found to compile");
    }

    // println!("Found {} source files to compile", source_files.len());
    println!(
        "{} {}Found {} files...",
        style("[2/4]").bold().dim(),
        LOOKING_GLASS,
        source_files.len()
    );

    let output_dir = build_config.output_dir.as_ref().unwrap();
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    compile_sources(&source_files, &build_config)?;
    Ok(())
}

pub fn discover_source_files(build_config: &BuildConfig) -> Result<Vec<PathBuf>> {
    let source_dir = build_config.source_dir.as_ref().unwrap();
    let mut source_files = Vec::new();

    let java_dir = Path::new(source_dir).join("java");
    let kotlin_dir = Path::new(source_dir).join("kotlin");

    if java_dir.exists() {
        collect_source_files(&java_dir, &mut source_files)?;
    }

    if kotlin_dir.exists() {
        collect_source_files(&kotlin_dir, &mut source_files)?;
    }

    // Fallback: if source_dir itself contains source files
    if source_files.is_empty() && Path::new(source_dir).exists() {
        collect_source_files(Path::new(source_dir), &mut source_files)?;
    }

    Ok(source_files)
}

pub fn collect_source_files(dir: &Path, acc: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_source_files(&path, acc)?;
        } else if path
            .extension()
            .is_some_and(|ext| ext == "java" || ext == "kt")
        {
            acc.push(path);
        }
    }

    Ok(())
}

pub fn compile_sources(source_files: &[PathBuf], build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    if let Ok(mut file) = std::fs::File::open("Cup.lock") {
        let tree = MerkleTree::builder("src/")
            .algorithm(merkle_hash::Algorithm::Blake3)
            .hash_names(false)
            .build()?;
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf);
        // dbg!(&buf);
        // dbg!(&tree.root.item.hash);

        if tree.root.item.hash == buf {
            return Ok(());
        }
    }

    let java_files: Vec<&PathBuf> = source_files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "java"))
        .collect();

    let kotlin_files: Vec<&PathBuf> = source_files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "kt"))
        .collect();

    let classpath = build_classpath();

    // If we have both Java and Kotlin files, we need to compile them in phases
    if !java_files.is_empty() && !kotlin_files.is_empty() {
        // println!("Compiling mixed Java/Kotlin project in two phases...");
        println!(
            "{} {}Compiling Java and Kotlin Files...",
            style("[3/4]").bold().dim(),
            LOOKING_GLASS
        );
        compile_mixed_project(
            &java_files,
            &kotlin_files,
            output_dir,
            &classpath,
            build_config,
        )?;
    } else if !kotlin_files.is_empty() {
        // println!("Compiling Kotlin files...");
        println!(
            "{} {}Compiling Kotlin Files...",
            style("[3/4]").bold().dim(),
            LOOKING_GLASS
        );
        compile_kotlin_files(&kotlin_files, output_dir, &classpath, build_config)?;
    } else if !java_files.is_empty() {
        println!(
            "{} {}Compiling Java Files...",
            style("[3/4]").bold().dim(),
            LOOKING_GLASS
        );
        compile_java_files(&java_files, output_dir, &classpath)?;
    }
    let tree = MerkleTree::builder("src/")
        .algorithm(merkle_hash::Algorithm::Blake3)
        .hash_names(false)
        .build()?;
    if let Ok(mut file) = std::fs::File::create("Cup.lock") {
        let _ = file
            .write(&tree.root.item.hash)
            .expect("error writing Cup.lock");
    }

    println!(
        "{} {}Compilation Successful!!!",
        style("[4/4]").bold().dim(),
        LOOKING_GLASS
    );

    Ok(())
}

fn compile_mixed_project(
    java_files: &[&PathBuf],
    kotlin_files: &[&PathBuf],
    output_dir: &str,
    classpath: &Option<String>,
    build_config: &BuildConfig,
) -> Result<()> {
    // println!("Compiling mixed Java/Kotlin project in two phases...");

    // println!("Phase 1: Compiling Kotlin files with Java source references...");
    let _ = compile_kotlin_with_java_sources(
        kotlin_files,
        java_files,
        output_dir,
        classpath,
        build_config,
    )
    .inspect_err(|e| eprintln!("{:?}", e));

    // println!("Phase 2: Compiling Java files with Kotlin classes in classpath...");
    let mut extended_classpath = vec![];

    // Add original classpath if it exists
    if let Some(cp) = classpath {
        extended_classpath.push(cp.clone());
    }

    // Add the output directory (containing compiled Kotlin classes) to classpath
    extended_classpath.push(output_dir.to_string());

    let combined_classpath = if !extended_classpath.is_empty() {
        Some(extended_classpath.join(if cfg!(windows) { ";" } else { ":" }))
    } else {
        Some(output_dir.to_string())
    };

    let _ = compile_java_files(java_files, output_dir, &combined_classpath)
        .inspect_err(|e| eprintln!("{:?}", e));

    // TODO: make a Cup.lock to hash the files and prevent successive needless compilations

    Ok(())
}

fn compile_kotlin_with_java_sources(
    kotlin_files: &[&PathBuf],
    java_files: &[&PathBuf],
    output_dir: &str,
    classpath: &Option<String>,
    _build_config: &BuildConfig,
) -> Result<()> {
    let mut cmd = Command::new("kotlinc");
    cmd.arg("-d").arg(output_dir);

    if let Some(cp) = classpath {
        cmd.arg("-cp").arg(cp);
    }

    // Add Kotlin files
    for file in kotlin_files {
        cmd.arg(file);
    }

    // Add Java files so kotlinc can reference them
    for file in java_files {
        cmd.arg(file);
    }

    let output = cmd
        .output()
        .context("Failed to execute kotlinc. Make sure Kotlin is installed and in PATH.")
        .inspect_err(|e| eprintln!("{e}"));

    if !output.unwrap().status.success() {
        // let stderr = String::from_utf8_lossy(&output.;
        bail!("Kotlin compilation with Java sources failed:\n{}", "a");
    }

    Ok(())
}

fn compile_kotlin_files(
    kotlin_files: &[&PathBuf],
    output_dir: &str,
    classpath: &Option<String>,
    _build_config: &BuildConfig,
) -> Result<()> {
    let mut cmd = Command::new("kotlinc");
    cmd.arg("-d").arg(output_dir);

    if let Some(cp) = classpath {
        cmd.arg("-cp").arg(cp);
    }

    for file in kotlin_files {
        cmd.arg(file);
    }

    let output = cmd
        .output()
        .context("Failed to execute kotlinc. Make sure Kotlin is installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Kotlin compilation failed:\n{}", stderr);
    }

    Ok(())
}

fn compile_java_files(
    java_files: &[&PathBuf],
    output_dir: &str,
    classpath: &Option<String>,
) -> Result<()> {
    let mut cmd = Command::new("javac");
    cmd.arg("-d").arg(output_dir);

    if let Some(cp) = classpath {
        cmd.arg("-cp").arg(cp);
    }

    for file in java_files {
        cmd.arg(file);
    }

    let output = cmd
        .output()
        .context("Failed to execute javac. Make sure Java is installed and in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Java compilation failed:\n{}", stderr);
    }

    Ok(())
}

pub fn build_classpath() -> Option<String> {
    let lib_dir = Path::new("lib");
    let mut jars = vec![];

    if lib_dir.exists()
        && let Ok(entries) = fs::read_dir(lib_dir)
    {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "jar") {
                jars.push(entry.path().to_string_lossy().to_string());
            }
        }
    }

    // Always include Kotlin runtime for Kotlin projects
    // Try to find Kotlin runtime in common locations
    let kotlin_runtime_paths = [
        "/usr/share/kotlin/lib/kotlin-stdlib.jar",
        "/opt/kotlin/lib/kotlin-stdlib.jar",
        "~/.kotlinc/lib/kotlin-stdlib.jar",
        "/usr/local/share/kotlin/lib/kotlin-stdlib.jar",
    ];

    for path in &kotlin_runtime_paths {
        if Path::new(path).exists() {
            jars.push(path.to_string());
            break;
        }
    }

    if !jars.is_empty() {
        Some(jars.join(if cfg!(windows) { ";" } else { ":" }))
    } else {
        None
    }
}
