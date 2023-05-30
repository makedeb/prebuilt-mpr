#![feature(async_closure)]
use clap::{Parser, Subcommand};
use log::LevelFilter;
use octocrab::Octocrab;
use std::{path::Path, process};

mod cache;
mod check_pkg;
mod pkglist;
mod run_checks;
mod upload_debs;
mod util;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The GitHub token to authenticate with.
    #[arg(long = "github-token", env = "GITHUB_TOKEN")]
    github_token: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Trigger GitHub Actions runs to check all packages for updates.
    RunChecks,
    /// Check if a package in the Prebuilt-MPR is out of date, creating a PR on
    /// GitHub if it is
    CheckPkg {
        /// The username of a GitHub account.
        #[arg(long = "github-username", env = "GITHUB_USERNAME")]
        github_username: String,
        /// The package to check
        pkg: String,
    },
    /// Upload the '.deb' files from a PR to the Prebuilt-MPR APT repository
    UploadDebs {
        /// The ProGet token to upload packages with.
        #[arg(long = "proget-token", env = "PROGET_TOKEN")]
        proget_token: String,
        /// The PR number to fetch the '.deb' files from
        pr: u64,
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
        .personal_token(cli.github_token.clone())
        .build()
        .unwrap();
    octocrab::initialise(crab);

    // Run the CLI.
    let exit_code = match cli.command {
        Command::RunChecks => run_checks::run_checks().await,
        Command::CheckPkg {
            github_username,
            pkg,
        } => check_pkg::check_pkg(&github_username, &cli.github_token, &pkg).await,
        Command::UploadDebs { pr, proget_token } => {
            upload_debs::upload_debs(pr, &proget_token).await
        }
    };
    process::exit(exit_code);
}
