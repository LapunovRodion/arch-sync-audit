use clap::{Parser, Subcommand};
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
            println!("export command");
            println!("output:{output}");
        }
        Commands::Check { profile } => {
            println!("check command");
            println!("profile :{profile}");
        }
        Commands::Plan { profile } => {
            println!("Plan command");
            println!("profile :{profile}");
        }
    }
}
