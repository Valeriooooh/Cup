use anyhow::{Context, Result, bail};
use std::fs;
use std::process::Command;

use crate::commands::build::compile_java_files;
use crate::commands::{discover_java_files, load_config};

use super::build::build_classpath;
use super::{BuildConfig};


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

    compile_java_files(&java_files, &build_config)?;

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


// // Enhanced file discovery for debugging/analysis
// #[derive(Debug, Clone)]
// pub enum FileType {
//     Directory(String),
//     JavaFile(String),
//     ClassFile(String),
//     JarFile(String),
//     Other(String),
// }

// impl FileType {
//     pub fn path(&self) -> &str {
//         match self {
//             FileType::Directory(path)
//             | FileType::JavaFile(path)
//             | FileType::ClassFile(path)
//             | FileType::JarFile(path)
//             | FileType::Other(path) => path,
//         }
//     }

//     pub fn from_path(path: &Path) -> Self {
//         let path_str = path.to_string_lossy().to_string();

//         if path.is_dir() {
//             return FileType::Directory(path_str);
//         }

//         match path.extension().and_then(|s| s.to_str()) {
//             Some("java") => FileType::JavaFile(path_str),
//             Some("class") => FileType::ClassFile(path_str),
//             Some("jar") => FileType::JarFile(path_str),
//             _ => FileType::Other(path_str),
//         }
//     }
// }

// pub fn analyze_project_structure() -> Result<Vec<FileType>> {
//     let mut items = Vec::new();
//     inspect_dir(Path::new("."), &mut items)?;

//     let mut seen = HashSet::new();
//     let filtered_items: Vec<FileType> = items
//         .into_iter()
//         .filter(|item| !item.path().contains(".git"))
//         .filter(|item| seen.insert(item.path().to_string()))
//         .collect();

//     Ok(filtered_items)
// }

// fn inspect_dir(path: &Path, acc: &mut Vec<FileType>) -> Result<()> {
//     let entries = match fs::read_dir(path) {
//         Ok(entries) => entries,
//         Err(_) => return Ok(()), // Skip unreadable directories
//     };

//     for entry in entries {
//         if let Ok(entry) = entry {
//             let entry_path = entry.path();
//             acc.push(FileType::from_path(&entry_path));

//             if entry_path.is_dir() {
//                 inspect_dir(&entry_path, acc)?;
//             }
//         }
//     }

//     Ok(())
// }
