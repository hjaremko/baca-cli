use crate::baca::{InstanceData, RequestBuilder};

mod logging_utils;
pub mod util;
mod view;

mod baca {
    use crate::util;
    use reqwest::blocking::Response;

    const BACA_HOST: &str = "baca.ii.uj.edu.pl";

    pub fn make_cookie() -> String {
        const VAR_NAME: &str = "BACA_SESSION";
        let cookie = util::get_env(VAR_NAME).expect("Session cookie not set!");

        format!("JSESSIONID={}; experimentation_subject_id=IjQ3YTM4YzY5LWI3NDItNDhjMS05MDJkLTIyYjIxZTlkNzZjYiI%3D--f329434f16371429c34e2e6eccd204760a89b9a9; acceptedCookies=yes; _ga=GA1.3.84942962.1597996247", cookie)
    }

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

    pub struct InstanceData {
        pub name: String,
        pub permutation: String,
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
    }

    pub struct RequestBuilder {
        client: reqwest::blocking::Client,
        data: InstanceData,
    }

    impl RequestBuilder {
        pub fn new(data: InstanceData) -> RequestBuilder {
            RequestBuilder {
                client: reqwest::blocking::Client::new(),
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
                .header(COOKIE, make_cookie())
                .header(CONTENT_TYPE, "text/x-gwt-rpc; charset=UTF-8")
                .header("DNT", "1")
                .header("X-GWT-Module-Base", self.data.make_module_base())
                .header("X-GWT-Permutation", &self.data.permutation)
        }
    }
}

fn main() {
    logging_utils::init_logging();

    // todo: read this from persistence
    let _nm = InstanceData {
        name: "mn2020".to_string(),
        permutation: "5A4AE95C27260DF45F17F9BF027335F6".to_string(),
    };

    let so = InstanceData {
        name: "so2018".to_string(),
        permutation: "022F1CFD68CBD2A9A4422647533A7495".to_string(),
    };

    let req = RequestBuilder::new(so);
    let submit_resp = req.send_submit_details("1943");

    tracing::info!("{:?}", submit_resp.text());
}
