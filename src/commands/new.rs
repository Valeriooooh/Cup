use std::{io::Write, process::Command};

pub fn new_project(project_name: String, location: Option<String>, kotlin: bool) {
    let full_path;
    if let Some(loc) = location {
        full_path = format!("{}/{}", loc, project_name);
    } else {
        full_path = project_name.to_string();
    }
    let _ = create_project_structure(full_path, project_name, kotlin);
}

fn create_project_structure(
    path: String,
    project_name: String,
    kotlin: bool,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&path)?;
    std::fs::create_dir_all(format!("{}/lib/", &path))?;
    std::fs::create_dir_all(format!("{}/build/", &path))?;
    std::fs::create_dir_all(format!("{}/doc/", &path))?;

    if kotlin {
        std::fs::create_dir_all(format!("{}/src/main/kotlin", &path))?;
        std::fs::create_dir_all(format!("{}/src/test/kotlin", &path))?;
        let mut main = std::fs::File::create_new(format!("{}/src/main/kotlin/Main.kt", &path))?;
        main.write_all(
            "package main

fun main() {
    println(\"Hello, World!\")
}
"
            .to_string()
            .as_bytes(),
        )?;
    } else {
        std::fs::create_dir_all(format!("{}/src/main/java", &path))?;
        std::fs::create_dir_all(format!("{}/src/test/java", &path))?;
        let mut main = std::fs::File::create_new(format!("{}/src/main/java/Main.java", &path))?;
        main.write_all(
            "package main;

public class Main {
    public static void main(String[] args) {
        System.out.println(\"Hello, World!\");
    }
}
"
            .to_string()
            .as_bytes(),
        )?;
    }

    let class_name = if kotlin { "MainKt" } else { "Main" };

    let mut toml = std::fs::File::create_new(format!("{}/Cup.toml", &path))?;
    toml.write_all(
        format!(
            "[project]
name = \"{}\"
version = \"0.1.0\"
main_class = \"main.{}\"

[build]
source_dir = \"src/main\"        # Optional: defaults to this
output_dir = \"build/classes\"    # Optional: defaults to this  
test_dir = \"src/test\"           # Optional: for future testing support
java_version = \"11\"             # Optional: for future version checking
doc_dir = \"doc\"                 # Optional: defaults to this

[dependencies]
",
            project_name, class_name
        )
        .as_bytes(),
    )?;

    let _ = Command::new("git")
        .arg("init")
        .current_dir(&path)
        .spawn()?
        .wait_with_output();

    Ok(())
}
