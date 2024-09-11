use super::input::ProjectDetails;
use std::env;
use std::process::{exit, Command};

impl ProjectDetails {
    pub fn create_project(&self) {
        let project_name = self.project_name.clone();
        let chosen_sdks = self.chosen_sdks.clone();

        // Step 1: Create a new cargo project
        println!("Creating a new cargo project: {}", project_name);
        let cargo_new_status = Command::new("cargo")
            .args(["new", &project_name])
            .status()
            .expect("Failed to create new cargo project");

        if !cargo_new_status.success() {
            eprintln!("Failed to create cargo project: {}", project_name);
            exit(1);
        }

        let project_dir = env::current_dir().unwrap().join(&project_name);
        let cargo_add = |args: Vec<&str>| {
            Command::new("cargo")
                .arg("add")
                .args(args.clone())
                .current_dir(&project_dir) // Set the working directory
                .status()
                .unwrap_or_else(|_| panic!("Failed to add {} to {project_name}", args[0]));
        };

        cargo_add(vec!["tokio", "--features=full"]);
        cargo_add(vec!["aws-config"]);
        chosen_sdks.iter().for_each(|sdk| cargo_add(vec![sdk]));
        println!(
            "Successfully created project '{}' and added dependencies.",
            project_name
        );
    }
}
