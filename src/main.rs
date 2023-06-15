use crate::cli::Cli;
use crate::update::{GithubReleases, UpdateCheckTimestamp, UpdateChecker, UpdateStatus};
use crate::workspace::{ConfigObject, WorkspaceDir};
use api::baca_service::BacaService;
use clap::Parser;
use colored::Colorize;
use std::env;
use tracing::{error, info, Level};

mod api;
mod cli;
mod command;
mod error;
mod log;
mod model;
mod parse;
mod update;
mod workspace;

fn main() {
    let cli = Cli::parse();
    let workspace = WorkspaceDir::new();
    let baca_api = BacaService::default();

    set_logging_level(&cli);
    check_for_updates(&workspace, cli.no_update, cli.force_update);

    let result = match &cli.command {
        Some(commands) => command::execute(&workspace, &baca_api, commands),
        None => Ok(()),
    };

    if let Err(e) = result {
        error!("{:?}", e);
        println!("{}", format!("{}", e).bright_red());
    }
}

fn set_logging_level(cli: &Cli) {
    let log_level = match cli.verbose {
        0 => return,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    log::init_logging(log_level);
}

fn check_for_updates(workspace: &WorkspaceDir, no_update: bool, force_update: bool) {
    if no_update {
        info!("Update check disabled.");
        return;
    }

    let now = UpdateCheckTimestamp::now();
    let last_check = UpdateCheckTimestamp::read_config(workspace).unwrap();

    if force_update || last_check.is_expired(&now) {
        let updates = fetch_updates();

        if let Err(e) = updates {
            error!("Error checking for updates: {}", e);
            return;
        }

        match updates.unwrap() {
            UpdateStatus::NoUpdates => {
                info!("No updates available.");

                now.save_config(workspace).unwrap_or_else(|e| {
                    error!("Error saving last update check timestamp: {:?}", e)
                });
            }
            UpdateStatus::Update(new_rel) => {
                println!(
                    "{}",
                    format!(
                        "New version {} is available!!\nDownload at {}",
                        new_rel.version, new_rel.link
                    )
                    .bright_yellow()
                )
            }
        }
    }
}

fn fetch_updates() -> error::Result<UpdateStatus> {
    let owner = env::var("GITHUB_USER").unwrap_or_else(|_| "hjaremko".to_string());
    let repo = env::var("GITHUB_REPO").unwrap_or_else(|_| "baca-cli".to_string());

    let gh_service = GithubReleases::new(&owner, &repo);
    let checker = UpdateChecker::new(gh_service, update::CURRENT_VERSION);
    checker.check_for_updates()
}
