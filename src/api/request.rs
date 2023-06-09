use reqwest::blocking::{multipart, RequestBuilder, Response};
use reqwest::header::{CONTENT_TYPE, COOKIE};
use tracing::{debug, info};

use crate::api::RequestType;
use crate::error;
use crate::error::Error;
use crate::model::Task;
use crate::workspace::ConnectionConfig;

pub struct Request<'a> {
    connection_config: &'a ConnectionConfig,
    client: reqwest::blocking::Client,
}

impl<'a> Request<'a> {
    pub fn new(connection_config: &'a ConnectionConfig) -> Self {
        Request {
            connection_config,
            client: reqwest::blocking::ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub fn login(self) -> reqwest::Result<Response> {
        let (login, pass) = self.connection_config.credentials();
        let req = self.make_request(RequestType::Login(login, pass));
        req.send()
    }

    pub fn details(self, id: &str) -> reqwest::Result<Response> {
        let req = self.make_request(RequestType::SubmitDetails(id.to_string()));
        req.send()
    }

    pub fn results(self) -> reqwest::Result<Response> {
        let req = self.make_request(RequestType::Results);
        req.send()
    }

    pub fn tasks(&self) -> reqwest::Result<Response> {
        let req = self.make_request(RequestType::Tasks);
        req.send()
    }

    pub fn submit(&self, task: &Task, file_path: &str) -> error::Result<Response> {
        let req = self.make_submit_request(task, file_path)?;
        req.send().map_err(|e| e.into())
    }

    pub fn allowed_languages(&self, task_id: &str) -> reqwest::Result<Response> {
        let req = self.make_request(RequestType::AllowedLanguages(task_id.to_string()));
        req.send()
    }

    fn make_request(&self, req_type: RequestType) -> RequestBuilder {
        let post_url = format!(
            "{}{}",
            self.connection_config.make_module_base(),
            req_type.mapping()
        );
        let payload = self.connection_config.make_payload(&req_type);

        info!("Making request to: {}", post_url);
        debug!("Request payload: {}", payload);

        let req = self.make_base_request(&post_url).body(payload);
        let req = match req_type {
            RequestType::Login(_, _) => req,
            _ => req.header(COOKIE, self.connection_config.make_cookie()),
        };

        debug!("{:?}", req);
        req
    }

    fn make_base_request(&self, url: &str) -> RequestBuilder {
        self.client
            .post(url)
            .header(CONTENT_TYPE, "text/x-gwt-rpc; charset=UTF-8")
            .header("DNT", "1")
            .header(
                "X-GWT-Module-Base",
                self.connection_config.make_module_base(),
            )
            .header("X-GWT-Permutation", &self.connection_config.permutation)
    }

    fn make_submit_request(&self, task: &Task, file_path: &str) -> error::Result<RequestBuilder> {
        let form = multipart::Form::new()
            .text("zadanie", task.id.clone())
            .text("jezyk", task.language.code())
            .file("zrodla", file_path)
            .map_err(|e| Error::ReadingSource(e.into()))?;

        let url = format!(
            "https://baca.ii.uj.edu.pl/{}/sendSubmit",
            self.connection_config.host
        );

        info!("Making submit request to: {}", url);
        debug!("Form: {:?}", form);

        let req = self
            .client
            .post(url)
            .multipart(form)
            .header(COOKIE, self.connection_config.make_cookie());
        debug!("{:?}", req);
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use super::*;

    fn make_connection_config() -> ConnectionConfig {
        ConnectionConfig {
            host: "mn2020".to_string(),
            login: "login".to_string(),
            password: "password".to_string(),
            permutation: "5A4AE95C27260DF45F17F9BF027335F6".to_string(),
            cookie: "cookie".to_string(),
        }
    }

    fn check_response(response: reqwest::Result<Response>) {
        if let Ok(response) = response {
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.text().unwrap(), "//OK[0,[],0,7]");
        };
    }

    #[test]
    #[ignore]
    fn login_should_connect() {
        let baca = make_connection_config();
        let req = Request::new(&baca);
        let response = req.login();

        check_response(response);
    }

    #[test]
    #[ignore]
    fn details_should_connect() {
        let baca = make_connection_config();
        let req = Request::new(&baca);
        let response = req.details("1");

        check_response(response);
    }

    #[test]
    #[ignore]
    fn results_should_connect() {
        let baca = make_connection_config();
        let req = Request::new(&baca);
        let response = req.results();

        check_response(response);
    }

    #[test]
    #[ignore]
    fn tasks_should_connect() {
        let baca = make_connection_config();
        let req = Request::new(&baca);
        let response = req.tasks();

        if let Ok(response) = response {
            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
