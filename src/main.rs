#[macro_use]
extern crate clap;
use clap::App;
use tracing::Level;

mod baca;
mod command;
mod log;
mod model;
mod parse;
mod workspace;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    log::init_logging(log_level);

    if let Some(matches) = matches.subcommand_matches("init") {
        let host = matches.value_of("host").unwrap();
        let login = matches.value_of("login").unwrap();
        let password = matches.value_of("password").unwrap();

        tracing::info!("Using BaCa host: {}", host);
        tracing::info!("Using BaCa login: {}", login);
        tracing::info!("Using BaCa password: {}", password);

        command::init(host, login, password);
        return; // todo: some error handling
    }

    if let Some(matches) = matches.subcommand_matches("details") {
        let submit_id = matches.value_of("id").unwrap();
        tracing::info!("Printing details for submit: {}", submit_id);

        command::details(submit_id);
        return;
    }

    if matches.subcommand_matches("refresh").is_some() {
        println!("Refreshing BaCa session...");
        command::refresh();
        return;
    }

    if let Some(matches) = matches.subcommand_matches("log") {
        let last_n = matches.value_of("amount").unwrap().parse().unwrap();
        command::log(last_n);
    }
}
