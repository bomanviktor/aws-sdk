use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    project_name: Option<String>,
}

pub struct ProjectDetails {
    pub project_name: String,
    pub chosen_sdks: Vec<String>,
}

impl ProjectDetails {
    pub fn new(aws_sdks: Vec<&str>) -> Self {
        let mut project_name: String;
        let mut chosen_sdks: Vec<String>;
        loop {
            let cli = Cli::parse();
            // Step 1: Ask for the project name if not provided as an argument
            project_name = cli.project_name.unwrap_or_else(|| {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the project name")
                    .interact_text()
                    .unwrap()
            });

            let selected_sdks = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select AWS SDKs (use space to select, enter to continue)")
                .items(&aws_sdks)
                .interact()
                .unwrap();

            // Step 3: Confirm and display the selected SDKs
            if selected_sdks.is_empty() {
                eprintln!("No SDKs selected. Exiting...");
                std::process::exit(1);
            }

            chosen_sdks = selected_sdks
                .into_iter()
                .map(|i| aws_sdks[i].to_string())
                .collect();

            if dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to proceed with these selections?")
                .interact()
                .unwrap()
            {
                break;
            }
        }
        ProjectDetails {
            project_name,
            chosen_sdks,
        }
    }
}
