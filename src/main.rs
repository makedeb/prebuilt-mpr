use clap::{Parser, Subcommand};
use log::LevelFilter;
use octocrab::Octocrab;
use std::{path::Path, process};

mod check_pkg;
mod pkglist;
mod run_checks;
mod util;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long = "github-token", env = "GITHUB_TOKEN")]
    github_token: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Trigger GitHub Actions runs to check all packages for updates.
    RunChecks,
    /// Check if a package in the Prebuilt-MPR is out of date, creating a PR on GitHub if it is
    CheckPkg {
        /// The package to check
        pkg: String,
    },
}

#[tokio::main]
async fn main() {
    // Load environment variables from the `.env` file if it exists.
    if Path::new(".env").exists() {
        dotenvy::dotenv().unwrap();
    }

    // Set up logging.
    log::set_logger(&hw_msg::HwLogger).unwrap();
    log::set_max_level(LevelFilter::Info);

    // Parse the CLI.
    let cli = Cli::parse();

    // Set up the global octocrab instance.
    let crab = Octocrab::builder()
        .personal_token(cli.github_token)
        .build()
        .unwrap();
    octocrab::initialise(crab);

    // Run the CLI.
    let exit_code = match cli.command {
        Command::RunChecks => run_checks::run_checks().await,
        Command::CheckPkg { pkg } => check_pkg::check_pkg(pkg).await,
    };
    process::exit(exit_code);
}
