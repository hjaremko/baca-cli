use clap::{App, AppSettings, Arg, SubCommand};
use tracing::Level;

mod commands;
mod logging_utils;
mod model;
mod persistence;
mod submit_parser;
pub mod util;

mod baca {
    use reqwest::blocking::Response;
    use reqwest::header::CONTENT_TYPE;
    use serde::{Deserialize, Serialize};

    const BACA_HOST: &str = "baca.ii.uj.edu.pl";

    pub enum RequestType {
        Results,
        SubmitDetails(String),
        Login(String, String),
    }

    impl RequestType {
        fn payload_template(&self) -> String {
            match self {
                RequestType::Results =>
                    "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getAllSubmits|Z|1|2|3|4|1|5|1|".to_string(),
                RequestType::SubmitDetails(id) =>
                    "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getSubmitDetails|I|1|2|3|4|1|5|".to_string() + id + "|",
                RequestType::Login(_, _) =>
                    "7|0|7|{}|620F3CE7784C04B839FC8E10C6C4A753|testerka.gwt.client.acess.PrivilegesService|login|java.lang.String/2004016611|{}|{}|1|2|3|4|2|5|5|6|7|".to_string(),
            }
        }

        fn mapping(&self) -> String {
            match *self {
                RequestType::Results => "submits".to_string(),
                RequestType::SubmitDetails(_) => "submits".to_string(),
                RequestType::Login(_, _) => "privileges".to_string(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct InstanceData {
        pub host: String,
        pub login: String,
        pub password: String,
        pub permutation: String,
        pub cookie: String,
    }

    impl InstanceData {
        pub fn make_url(&self) -> String {
            format!("https://{}/{}", BACA_HOST, self.host)
        }

        pub fn make_module_base(&self) -> String {
            format!("{}/testerka_gwt/", self.make_url())
        }

        pub fn make_payload(&self, req_type: RequestType) -> String {
            use dyn_fmt::AsStrFormatExt;

            req_type.payload_template().format(&[
                self.make_module_base(),
                self.login.clone(),
                self.password.clone(),
            ])
        }

        pub fn make_cookie(&self) -> String {
            format!("JSESSIONID={}; experimentation_subject_id=IjQ3YTM4YzY5LWI3NDItNDhjMS05MDJkLTIyYjIxZTlkNzZjYiI%3D--f329434f16371429c34e2e6eccd204760a89b9a9; acceptedCookies=yes; _ga=GA1.3.84942962.1597996247", self.cookie)
        }
    }

    pub struct RequestBuilder {
        client: reqwest::blocking::Client,
        data: InstanceData,
    }

    impl RequestBuilder {
        pub fn new(data: InstanceData) -> RequestBuilder {
            RequestBuilder {
                client: reqwest::blocking::ClientBuilder::new()
                    .danger_accept_invalid_certs(true)
                    .build()
                    .unwrap(),
                data,
            }
        }

        pub fn send_login(&self) -> Response {
            self.make_login_request(RequestType::Login(
                (&self.data.login).to_string(),
                (&self.data.password).to_string(),
            ))
            .send()
            .unwrap()
        }

        pub fn send_results(&self) -> Response {
            self.make_request(RequestType::Results).send().unwrap()
        }

        pub fn send_submit_details(&self, submit_id: &str) -> Response {
            let req = self.make_request(RequestType::SubmitDetails(submit_id.to_string()));

            tracing::debug!("{:?}", req);

            req.send().unwrap()
        }

        fn make_login_request(&self, req_type: RequestType) -> reqwest::blocking::RequestBuilder {
            let post_url = format!("{}{}", self.data.make_module_base(), req_type.mapping());
            let payload = self.data.make_payload(req_type);

            tracing::debug!("Making login request with payload: {}", payload);

            self.client
                .post(post_url)
                .body(payload)
                .header(CONTENT_TYPE, "text/x-gwt-rpc; charset=UTF-8")
                .header("DNT", "1")
                .header("X-GWT-Module-Base", self.data.make_module_base())
                .header("X-GWT-Permutation", &self.data.permutation)
        }

        fn make_request(&self, req_type: RequestType) -> reqwest::blocking::RequestBuilder {
            use reqwest::header::COOKIE;
            let post_url = format!("{}{}", self.data.make_module_base(), req_type.mapping());
            let payload = self.data.make_payload(req_type);

            tracing::debug!("Making request to: {}", post_url);
            tracing::debug!("Making request with payload: {}", payload);

            self.client
                .post(post_url)
                .body(payload)
                .header(COOKIE, self.data.make_cookie())
                .header(CONTENT_TYPE, "text/x-gwt-rpc; charset=UTF-8")
                .header("DNT", "1")
                .header("X-GWT-Module-Base", self.data.make_module_base())
                .header("X-GWT-Permutation", &self.data.permutation)
        }
    }
}

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
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        3 | _ => Level::TRACE,
    };

    logging_utils::init_logging(log_level);

    if let Some(matches) = matches.subcommand_matches("init") {
        let host = matches.value_of("host").unwrap();
        let login = matches.value_of("login").unwrap();
        let password = matches.value_of("password").unwrap();

        tracing::info!("Using BaCa host: {}", host);
        tracing::info!("Using BaCa login: {}", login);
        tracing::info!("Using BaCa password: {}", password);

        commands::init(host, login, password);
        return; // todo: some error handling
    }

    if let Some(matches) = matches.subcommand_matches("details") {
        let submit_id = matches.value_of("id").unwrap();
        tracing::info!("Printing details for submit: {}", submit_id);

        commands::submit_details(submit_id);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("refresh") {
        println!("Refreshing BaCa session...");
        commands::refresh();
        return;
    }
}
