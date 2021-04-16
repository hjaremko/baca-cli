use clap::{App, AppSettings, Arg, SubCommand};
use tracing::Level;

mod baca;
mod command;
mod log;
mod model;
mod parse;
mod workspace;

fn main() {
    // todo: from yaml
    let matches = App::new("BaCa CLI")
        .version("1.0.0")
        .author("Hubert Jaremko <hjaremko@outlook.com>")
        .about("CLI client for the Jagiellonian University's BaCa online judge")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes current directory as BaCa workspace")
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .help("BaCa hostname, ex. mn2020")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("login")
                        .short("l")
                        .long("login")
                        .help("BaCa login")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("BaCa password")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("details")
                .about("Gets submit details")
                .setting(AppSettings::AllowMissingPositional)
                .arg(Arg::with_name("id").required(true)),
        )
        .subcommand(
            SubCommand::with_name("refresh")
                .about("Refreshes session, use in case of cookie expiration"),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Prints last (default: 3) submits")
                .setting(AppSettings::AllowMissingPositional)
                .arg(
                    Arg::with_name("amount")
                        .help("Amount of last submits to print")
                        .default_value("3")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
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
