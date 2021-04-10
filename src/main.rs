use crate::baca::{InstanceData, RequestBuilder};
use crate::submit_parser::SubmitParser;
use clap::{App, AppSettings, Arg, SubCommand};
use std::rc::Rc;
use tracing::Level;

mod commands;
mod logging_utils;
mod model;
mod persistence;
mod submit_parser;
pub mod util;

mod baca {
    use reqwest::blocking::Response;
    use serde::{Deserialize, Serialize};
    use std::rc::Rc;

    const BACA_HOST: &str = "baca.ii.uj.edu.pl";

    pub enum RequestType {
        Results,
        SubmitDetails(String),
    }

    impl RequestType {
        fn payload_template(&self) -> String {
            match self {
                RequestType::Results =>
                    "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getAllSubmits|Z|1|2|3|4|1|5|1|".to_string(),
                RequestType::SubmitDetails(id) =>
                    "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getSubmitDetails|I|1|2|3|4|1|5|".to_string() + id + "|",
            }
        }

        fn mapping(&self) -> String {
            match *self {
                RequestType::Results => "submits".to_string(),
                RequestType::SubmitDetails(_) => "submits".to_string(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InstanceData {
        pub name: String,
        pub permutation: String,
        pub cookie: String,
    }

    impl InstanceData {
        pub fn make_url(&self) -> String {
            format!("https://{}/{}", BACA_HOST, self.name)
        }

        pub fn make_module_base(&self) -> String {
            format!("{}/testerka_gwt/", self.make_url())
        }

        pub fn make_payload(&self, req_type: RequestType) -> String {
            use dyn_fmt::AsStrFormatExt;

            req_type
                .payload_template()
                .format(&[self.make_module_base()])
        }

        pub fn make_cookie(&self) -> String {
            format!("JSESSIONID={}; experimentation_subject_id=IjQ3YTM4YzY5LWI3NDItNDhjMS05MDJkLTIyYjIxZTlkNzZjYiI%3D--f329434f16371429c34e2e6eccd204760a89b9a9; acceptedCookies=yes; _ga=GA1.3.84942962.1597996247", self.cookie)
        }
    }

    pub struct RequestBuilder {
        client: reqwest::blocking::Client,
        data: Rc<InstanceData>,
    }

    impl RequestBuilder {
        pub fn new(data: Rc<InstanceData>) -> RequestBuilder {
            RequestBuilder {
                client: reqwest::blocking::ClientBuilder::new()
                    .danger_accept_invalid_certs(true)
                    .build()
                    .unwrap(),
                data,
            }
        }

        pub fn send_results(&self) -> Response {
            self.make_request(RequestType::Results).send().unwrap()
        }

        pub fn send_submit_details(&self, submit_id: &str) -> Response {
            self.make_request(RequestType::SubmitDetails(submit_id.to_string()))
                .send()
                .unwrap()
        }

        fn make_request(&self, req_type: RequestType) -> reqwest::blocking::RequestBuilder {
            use reqwest::header::{CONTENT_TYPE, COOKIE};

            let post_url = format!("{}{}", self.data.make_module_base(), req_type.mapping());

            self.client
                .post(post_url)
                .body(self.data.make_payload(req_type))
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
                .arg(Arg::with_name("permutation")
                    .short("p")
                    .long("perm")
                    .help("BaCa host permutation, found in 'X-GWT-Permutation' header of HTTP request")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("session")
                    .short("s")
                    .long("session")
                    .help("BaCa session cookie, found in 'JSESSIONID' cookie of HTTP request")
                    .required(true)
                    .takes_value(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("details")
                .about("Gets submit details")
                .setting(AppSettings::AllowMissingPositional)
                .arg(
                    Arg::with_name("id")
                        .required(true),
                ),
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
        let perm = matches.value_of("permutation").unwrap();
        let cookie = matches.value_of("session").unwrap();

        tracing::info!("Using BaCa host: {}", host);
        tracing::info!("Using BaCa permutation: {}", perm);
        tracing::info!("Using BaCa session cookie: {}", cookie);

        commands::init(host, perm, cookie);
        return; // todo: some error handling
    }

    if let Some(matches) = matches.subcommand_matches("details") {
        let submit_id = matches.value_of("id").unwrap();
        tracing::info!("Printing details for submit: {}", submit_id);

        commands::submit_details(submit_id);
        return;
    }
}
