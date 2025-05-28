use std::{error::Error, io::Write, process::Command};

use dialoguer::{theme::ColorfulTheme, Completion, Input};


pub fn new_project(project_name: String, location: Option<String>) {
    // let _ = Command::new("java").arg("-version").spawn();
    let completion = NewProjectCompletion::default();
    let mut triple: Vec<&str>;
    let mut project_structure;
    loop{
        project_structure = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Project structure: ")
            .with_initial_text(format!("org.mycompany.{}",project_name))
            .interact_text()
            .unwrap();
        triple = project_structure.trim().split(".").clone().collect();
        println!("{:?}",triple);
        if triple.len() != 3 {
            println!("Invalid project structure");
            continue;
        }
        break;
    }
    let mut full_path;
    if let Some(loc) = location {
        full_path = format!("{}/{}",loc,project_name);
    }else{
        full_path = format!("{}",project_name);
    }
    let triple = triple.join("/");
    let _ = create_project_structure(full_path, triple);
}


fn create_project_structure(path: String, triple: String) -> Result<(), std::io::Error>{
    let _ = std::fs::create_dir_all(&path)?;
    let _ = std::fs::create_dir_all(format!("{}/src/{}",&path, &triple))?;
    let mut main = std::fs::File::create_new(format!("{}/src/{}/Main.java",&path, &triple))?;
    let _ = main.write_all("

".as_bytes())?;
    let _ = std::fs::create_dir_all(format!("{}/bin/{}",&path, &triple))?;

    Ok(())
}


struct NewProjectCompletion {
    options: Vec<String>,
}

impl Default for NewProjectCompletion {
    fn default() -> Self {
        NewProjectCompletion {
            options: vec![
                "example".to_string(),
                "com".to_string(),
                "org".to_string(),
            ],
        }
    }
}

impl Completion for NewProjectCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
