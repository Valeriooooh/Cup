use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use toml::Table;

use anyhow::{Context, Result, bail};
pub mod build;
pub mod doc;
pub mod new;
pub mod run;

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
    pub project_lang: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    pub source_dir: Option<String>,
    pub output_dir: Option<String>,
    pub test_dir: Option<String>,
    pub java_version: Option<String>,
    pub doc_dir: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            source_dir: Some("src/main".to_string()), // Changed to match your config
            output_dir: Some("build/classes".to_string()),
            test_dir: Some("src/test".to_string()),
            java_version: Some("11".to_string()),
            doc_dir: Some("doc".to_string()),
        }
    }
}

pub fn load_config() -> Result<CupConfig> {
    let config_content = fs::read_to_string("Cup.toml")
        .context("Failed to read Cup.toml. Make sure it exists in the project root.")?;
    let config: CupConfig = toml::from_str(&config_content).context("failed to import config")?;

    Ok(config)
}

// Keep the old function for backward compatibility with doc.rs
pub fn discover_java_files(build_config: &BuildConfig) -> Result<Vec<PathBuf>> {
    crate::commands::build::discover_source_files(build_config)
}

pub fn collect_java_files(dir: &Path, acc: &mut Vec<PathBuf>) -> Result<()> {
    crate::commands::build::collect_source_files(dir, acc)
}
