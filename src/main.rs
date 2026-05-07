use clap::{Parser, Subcommand};
use profile::SystemProfile;
use std::fs;
mod compare;
mod profile;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Export {
        #[arg(short, long)]
        output: String,
    },
    Check {
        profile: String,
    },
    Plan {
        profile: String,
    },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export { output } => {
            let profile = SystemProfile {
                pacman_packages: vec!["neovim".to_string(), "ripgrep".to_string()],
                aur_packages: vec!["visual-studio-code-bin".to_string()],
                shell: Some("/bin/zsh".to_string()),
                config_paths: vec!["~/.zshrc".to_string()],
                system_services: vec![],
                user_services: vec![],
            };
            let toml = toml::to_string_pretty(&profile).unwrap();
            fs::write(output, toml).unwrap()
        }
        Commands::Check { profile } => {
            let content = fs::read_to_string(profile).unwrap();
            let profile: SystemProfile = toml::from_str(&content).unwrap();
            println!("{profile:#?}");
        }
        Commands::Plan { profile } => {
            println!("Plan command");
            println!("profile :{profile}");
        }
    }
}
