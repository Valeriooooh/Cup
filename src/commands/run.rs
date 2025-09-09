use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Table;

#[derive(Debug, Deserialize, Serialize)]
pub struct CupConfig {
    pub project: ProjectConfig,
    pub build: Option<BuildConfig>,
    pub dependencies: Option<Table>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub main_class: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    pub source_dir: Option<String>,
    pub output_dir: Option<String>,
    pub test_dir: Option<String>,
    pub java_version: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            source_dir: Some("src/main/java".to_string()),
            output_dir: Some("build/classes".to_string()),
            test_dir: Some("src/test/java".to_string()),
            java_version: Some("11".to_string()),
        }
    }
}

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

    // Create output directory
    let output_dir = build_config.output_dir.as_ref().unwrap();
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    // Compile Java files
    compile_java_files(&java_files, &build_config)?;

    // Run the main class if specified
    if let Some(main_class) = &config.project.main_class {
        println!("Running main class: {}", main_class);
        run_main_class(main_class, &build_config)?;
    } else {
        println!("No main class specified in Cup.toml. Compilation complete.");
    }

    Ok(())
}

fn load_config() -> Result<CupConfig> {
    let config_content = fs::read_to_string("./Cup.toml")
        .context("Failed to read Cup.toml. Make sure it exists in the project root.")?;
    let config: CupConfig = toml::from_str(&config_content).context("failed to import config")?;

    Ok(config)
}

fn discover_java_files(build_config: &BuildConfig) -> Result<Vec<PathBuf>> {
    let source_dir = build_config.source_dir.as_ref().unwrap();
    let mut java_files = Vec::new();

    if !Path::new(source_dir).exists() {
        bail!("Source directory '{}' does not exist", source_dir);
    }

    collect_java_files(Path::new(source_dir), &mut java_files)?;
    Ok(java_files)
}

fn collect_java_files(dir: &Path, acc: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_java_files(&path, acc)?;
        } else if path.extension().map_or(false, |ext| ext == "java") {
            acc.push(path);
        }
    }

    Ok(())
}

fn compile_java_files(java_files: &[PathBuf], build_config: &BuildConfig) -> Result<()> {
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

fn run_main_class(main_class: &str, build_config: &BuildConfig) -> Result<()> {
    let output_dir = build_config.output_dir.as_ref().unwrap();

    let mut cmd = Command::new("java");
    cmd.arg("-cp").arg(output_dir);

    // Add classpath for dependencies
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

fn build_classpath() -> Option<String> {
    // Placeholder for dependency management
    // In the future, this could download JARs, resolve Maven dependencies, etc.
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
            return Some(jars.join(":"));
        }
    }
    None
}

// Enhanced file discovery for debugging/analysis
#[derive(Debug, Clone)]
pub enum FileType {
    Directory(String),
    JavaFile(String),
    ClassFile(String),
    JarFile(String),
    Other(String),
}

impl FileType {
    pub fn path(&self) -> &str {
        match self {
            FileType::Directory(path)
            | FileType::JavaFile(path)
            | FileType::ClassFile(path)
            | FileType::JarFile(path)
            | FileType::Other(path) => path,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let path_str = path.to_string_lossy().to_string();

        if path.is_dir() {
            return FileType::Directory(path_str);
        }

        match path.extension().and_then(|s| s.to_str()) {
            Some("java") => FileType::JavaFile(path_str),
            Some("class") => FileType::ClassFile(path_str),
            Some("jar") => FileType::JarFile(path_str),
            _ => FileType::Other(path_str),
        }
    }
}

pub fn analyze_project_structure() -> Result<Vec<FileType>> {
    let mut items = Vec::new();
    inspect_dir(Path::new("."), &mut items)?;

    // Remove duplicates and filter out .git
    let mut seen = HashSet::new();
    let filtered_items: Vec<FileType> = items
        .into_iter()
        .filter(|item| !item.path().contains(".git"))
        .filter(|item| seen.insert(item.path().to_string()))
        .collect();

    Ok(filtered_items)
}

fn inspect_dir(path: &Path, acc: &mut Vec<FileType>) -> Result<()> {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Ok(()), // Skip unreadable directories
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            acc.push(FileType::from_path(&entry_path));

            if entry_path.is_dir() {
                inspect_dir(&entry_path, acc)?;
            }
        }
    }

    Ok(())
}
