use std::process::Command;

pub fn new_project(project_name: String, location: Option<String>) {
    let _ = Command::new("java").arg("-version").spawn();
}
