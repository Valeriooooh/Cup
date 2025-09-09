use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use toml::Table;

use anyhow::{Context, Result, bail};
pub mod new;
pub mod run;
pub mod build;


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

pub fn load_config() -> Result<CupConfig> {
    let config_content = fs::read_to_string("Cup.toml")
        .context("Failed to read Cup.toml. Make sure it exists in the project root.")?;
    let config: CupConfig = toml::from_str(&config_content).context("failed to import config")?;

    Ok(config)
}

pub fn discover_java_files(build_config: &BuildConfig) -> Result<Vec<PathBuf>> {
    let source_dir = build_config.source_dir.as_ref().unwrap();
    let mut java_files = Vec::new();

    if !Path::new(source_dir).exists() {
        bail!("Source directory '{}' does not exist", source_dir);
    }

    collect_java_files(Path::new(source_dir), &mut java_files)?;
    Ok(java_files)
}

pub fn collect_java_files(dir: &Path, acc: &mut Vec<PathBuf>) -> Result<()> {
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
