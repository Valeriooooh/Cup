use serde::Deserialize;

pub mod new;
pub mod run;

#[derive(Deserialize)]
pub struct Project{
    pub name: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct Config{
    project: Project
}
