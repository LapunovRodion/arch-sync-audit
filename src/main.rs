use clap::{Parser, Subcommand};
use profile::SystemProfile;
use std::fs;
mod compare;
mod profile;
mod collect;
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
                shell: collect:current_shell(),
                config_paths: vec!["~/.zshrc".to_string()],
                system_services: vec![],
                user_services: vec![],
            };
            let toml = toml::to_string_pretty(&profile).unwrap();
            fs::write(output, toml).unwrap()
        }
        Commands::Check { profile } => {
            let content = fs::read_to_string(profile).unwrap();
            let expected: SystemProfile = toml::from_str(&content).unwrap();
            let current = SystemProfile{
                pacman_packages:vec!["neovim".to_string()],
                aur_packages:vec![],
                shell:Some("/bin/bash".to_string()),
                config_paths: vec![],
                system_services: vec![],

                user_services: vec![],
            };
            let diff = compare::compare_profiles(&expected, &current);
            if !diff.missing_pacman_packages.is_empty()
            {
                println!("Missing pacman packages:");
                for package in &diff.missing_pacman_packages{
                    println!("- {package}");
                }
                println!();
            }
            if !diff.missing_aur_packages.is_empty()
            {
                println!("Missing AUR packages:");
                for package in &diff.missing_aur_packages{
                    println!("- {package}");
                }
                println!();
            }
        if let Some(shell_diff) = diff.shell_diff{
            println!("Different shell:");
            println!("expected: {}",shell_diff.expected);
            println!(
                "current: {}",
                shell_diff.current.unwrap_or_else(|| "unknown".to_string())
            );
        }


        }
        Commands::Plan { profile } => {
            println!("Plan command");
            println!("profile :{profile}");
        }
    }
}
