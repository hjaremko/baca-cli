#[macro_use]
extern crate clap;
use crate::baca::details::Language;
use crate::update::{GithubReleases, UpdateChecker, UpdateStatus};
use crate::workspace::TaskConfig;
use clap::{App, AppSettings};
use colored::Colorize;
use std::path::Path;
use std::str::FromStr;
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

    // todo: -r to override current config
    if let Some(matches) = matches.subcommand_matches("init") {
        let host = matches.value_of("host").unwrap();
        let login = matches.value_of("login").unwrap();
        let password = matches.value_of("password").unwrap();

        // todo: if error remove dir
        if let Err(e) = command::init(host, login, password) {
            println!("{}", format!("{}", e).bright_red());
        }
        return;
    }

    // todo: print test logs as well
    if let Some(matches) = matches.subcommand_matches("details") {
        let submit_id = matches.value_of("id").unwrap();

        if let Err(e) = command::details(submit_id) {
            println!("{}", format!("{}", e).bright_red());
        }
        return;
    }

    if matches.subcommand_matches("refresh").is_some() {
        if let Err(e) = command::refresh() {
            println!("{}", format!("{}", e).bright_red());
        }
        return;
    }

    // if task is configured, filter logs, add --all switch
    if let Some(matches) = matches.subcommand_matches("log") {
        let last_n = matches.value_of("amount").unwrap().parse::<usize>();
        let last_n = match last_n {
            Ok(n) => n,
            Err(_) => {
                println!("{}", "Invalid log argument.".bright_red());
                return;
            }
        };

        if let Err(e) = command::log(last_n) {
            println!("{}", format!("{}", e).bright_red());
        }
        return;
    }

    if matches.subcommand_matches("tasks").is_some() {
        if let Err(e) = command::tasks() {
            println!("{}", format!("{}", e).bright_red());
        }
        return; // todo: return error
    }

    if let Some(matches) = matches.subcommand_matches("submit") {
        if matches.subcommand_matches("clear").is_some() {
            if let Err(e) = workspace::remove_task() {
                println!("{}", format!("{}", e).bright_red());
            }
            return;
        }

        let task_id = matches.value_of("task_id");
        let file_path = matches.value_of("file");
        let to_zip = matches.is_present("zip");
        let lang = matches.value_of("language");
        let saved = workspace::read_task();

        if let Some(lang) = lang {
            if let Ok(Language::Unsupported) = Language::from_str(lang) {
                println!("{} {}", lang, "is not yet supported!! Please create an issue at https://github.com/hjaremko/baca-cli/issues".bright_red());
                return;
            }
        }

        if saved.is_err() {
            info!("Task not loaded.");
        }

        if task_id.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide task_id. Type 'baca submit -h' for more info.".bright_red()
            );
            return;
        }

        if file_path.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide file. Type 'baca submit -h' for more info.".bright_red()
            );
            return;
        }

        if lang.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide language. Type 'baca submit -h' for more info.".bright_red()
            );
            return;
        }

        // todo: default()
        let saved = saved.unwrap_or(TaskConfig {
            id: "".to_string(),
            file: "".to_string(),
            to_zip: false,
            language: Language::Unsupported,
        });

        let task_id = match task_id {
            None => saved.id.clone(),
            Some(id) => id.to_string(),
        };

        let file_path = match file_path {
            None => saved.file.clone(),
            Some(file) => file.to_string(),
        };

        let lang = match lang {
            None => saved.language,
            Some(lang) => Language::from_str(lang).unwrap(),
        };

        let to_zip = match to_zip {
            true => true,
            false => saved.to_zip,
        };

        if matches.is_present("default") {
            if let Err(e) = workspace::save_task(&task_id, &file_path, to_zip, lang) {
                println!("{}", format!("{}", e).bright_red());
            }
        }

        let file_to_submit = if to_zip {
            let path = Path::new(&file_path);
            let res = workspace::zip_file(path);

            if let Err(e) = res {
                println!(
                    "Error zipping {}! Error: {}",
                    path.to_str().unwrap(),
                    e.to_string().bright_red()
                );
                return;
            }

            res.unwrap()
        } else {
            file_path
        };

        if let Err(e) = command::submit(&task_id, file_to_submit.as_str(), &lang) {
            println!("{}", format!("{}", e).bright_red());
        }
        return;
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
