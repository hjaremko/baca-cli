#[macro_use]
extern crate clap;

#[cfg(test)]
#[macro_use]
extern crate serial_test;

use crate::baca::api::baca_service::BacaService;
use crate::update::{GithubReleases, UpdateChecker, UpdateStatus};

use crate::workspace::WorkspaceDir;
use clap::{App, AppSettings};
use colored::Colorize;
use tracing::{error, info, Level};

mod baca;
mod command;
mod error;
mod log;
mod model;
mod parse;
mod update;
mod workspace;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();

    let verbose_matches = matches.occurrences_of("verbose");

    let log_level = match verbose_matches {
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    if verbose_matches != 0 {
        log::init_logging(log_level);
    }

    check_for_updates();

    if let (command, Some(sub_matches)) = matches.subcommand() {
        if let Err(e) = command::execute::<WorkspaceDir, BacaService>(command, sub_matches) {
            println!("{}", format!("{}", e).bright_red());
        }
    }
}

fn check_for_updates() {
    let gh_service = GithubReleases::new("hjaremko", "baca-cli");
    let checker = UpdateChecker::new(gh_service, update::CURRENT_VERSION);
    let status = checker.check_for_updates();

    if let Err(e) = status {
        error!("Error checking for updates: {}", e);
        return;
    }

    match status.unwrap() {
        UpdateStatus::NoUpdates => info!("No updates available."),
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
    };
}
