use anyhow::Result;
use clap::{Parser, Subcommand};
use profile::SystemProfile;
use std::fs;
mod collect;
mod compare;
mod plan;
mod profile;
mod report;
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
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export { output } => {
            let aur_packages = collect::aur_packages()?;
            let pacman_packages = collect::pacman_packages(&aur_packages)?;
            let profile = SystemProfile {
                pacman_packages,
                aur_packages,
                shell: collect::current_shell(),
                config_paths: collect::config_paths(),
                system_services: collect::system_services()?,
                user_services: collect::user_services(),
            };
            let toml = toml::to_string_pretty(&profile)?;
            fs::write(output, toml)?;
        }
        Commands::Check { profile } => {
            let content = fs::read_to_string(profile)?;
            let expected: SystemProfile = toml::from_str(&content)?;
            let current = collect::current_profile()?;
            let diff = compare::compare_profiles(&expected, &current);
            println!("{}", report::render_report(&diff));
        }
        Commands::Plan { profile } => {
            let content = fs::read_to_string(profile)?;
            let expected: SystemProfile = toml::from_str(&content)?;
            let current = collect::current_profile()?;
            let diff = compare::compare_profiles(&expected, &current);
            println!("{}", plan::render_plan(&diff));
        }
    }

    Ok(())
}
