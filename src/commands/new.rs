use std::{io::Write, process::Command};

// use dialoguer::{theme::ColorfulTheme, Completion, Input};

pub fn new_project(project_name: String, location: Option<String>) {
    // let completion = NewProjectCompletion::default();
    // let mut triple: Vec<&str>;
    // let mut project_structure;
    // loop{
    //     project_structure = Input::<String>::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Project structure: ")
    //         .with_initial_text(format!("org.mycompany.{}",project_name))
    //         .interact_text()
    //         .unwrap();
    //     triple = project_structure.trim().split(".").clone().collect();
    //     // println!("{:?}",triple);
    //     if triple.len() != 3 {
    //         println!("Invalid project structure");
    //         continue;
    //     }
    //     break;
    // }
    let full_path;
    if let Some(loc) = location {
        full_path = format!("{}/{}", loc, project_name);
    } else {
        full_path = project_name.to_string();
    }
    let _ = create_project_structure(full_path, project_name);
}

fn create_project_structure(path: String, project_name: String) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&path)?;
    std::fs::create_dir_all(format!("{}/src/main/java", &path))?;
    std::fs::create_dir_all(format!("{}/src/test/java", &path))?;
    std::fs::create_dir_all(format!("{}/lib/", &path))?;
    std::fs::create_dir_all(format!("{}/build/", &path))?;
    std::fs::create_dir_all(format!("{}/doc/", &path))?;
    let mut main = std::fs::File::create_new(format!("{}/src/main/java/Main.java", &path))?;
    //let package = vec![triple.get(0).unwrap().to_owned(), triple.get(1).unwrap().to_owned()].join(".");
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
    let mut toml = std::fs::File::create_new(format!("{}/Cup.toml", &path))?;
    toml.write_all(
        format!(
            "[project]
name = \"{}\"
version = \"0.1.0\"
main_class = \"main.Main\"
[build]

source_dir = \"src/main/java\"    # Optional: defaults to this
output_dir = \"build/classes\"     # Optional: defaults to this  
test_dir = \"src/test/java\"       # Optional: for future testing support
java_version = \"11\"              # Optional: for future version checking

[dependencies]
",
            project_name
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

// struct NewProjectCompletion {
//     options: Vec<String>,
// }

// impl Default for NewProjectCompletion {
//     fn default() -> Self {
//         NewProjectCompletion {
//             options: vec![
//                 "example".to_string(),
//                 "com".to_string(),
//                 "org".to_string(),
//             ],
//         }
//     }
// }

// impl Completion for NewProjectCompletion {
//     /// Simple completion implementation based on substring
//     fn get(&self, input: &str) -> Option<String> {
//         let matches = self
//             .options
//             .iter()
//             .filter(|option| option.starts_with(input))
//             .collect::<Vec<_>>();

//         if matches.len() == 1 {
//             Some(matches[0].to_string())
//         } else {
//             None
//         }
//     }
// }
