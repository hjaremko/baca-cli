use crate::baca::api::RequestType;
use crate::workspace::InstanceData;
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{CONTENT_TYPE, COOKIE};

pub struct Request<'a> {
    instance: &'a InstanceData,
    client: reqwest::blocking::Client,
}

impl<'a> Request<'a> {
    pub fn new(instance: &'a InstanceData) -> Self {
        Request {
            instance,
            client: reqwest::blocking::ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub fn login(self) -> reqwest::Result<Response> {
        let (login, pass) = self.instance.credentials();
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

    // todo: submit here

    fn make_request(&self, req_type: RequestType) -> RequestBuilder {
        let post_url = format!("{}{}", self.instance.make_module_base(), req_type.mapping());
        let payload = self.instance.make_payload(&req_type);

        tracing::info!("Making request to: {}", post_url);
        tracing::debug!("Request payload: {}", payload);

        let req = self.make_base_request(&post_url).body(payload);
        let req = match req_type {
            RequestType::Login(_, _) => req,
            _ => req.header(COOKIE, self.instance.make_cookie()),
        };

        tracing::debug!("{:?}", req);
        req
    }

    fn make_base_request(&self, url: &str) -> RequestBuilder {
        self.client
            .post(url)
            .header(CONTENT_TYPE, "text/x-gwt-rpc; charset=UTF-8")
            .header("DNT", "1")
            .header("X-GWT-Module-Base", self.instance.make_module_base())
            .header("X-GWT-Permutation", &self.instance.permutation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    fn make_instance() -> InstanceData {
        InstanceData {
            host: "mn2020".to_string(),
            login: "login".to_string(),
            password: "password".to_string(),
            permutation: "5A4AE95C27260DF45F17F9BF027335F6".to_string(),
            cookie: "cookie".to_string(),
        }
    }

    fn check_response(response: reqwest::Result<Response>) {
        // todo: tsl error
        match response {
            Ok(response) => {
                assert_eq!(response.status(), StatusCode::OK);
                assert_eq!(response.text().unwrap(), "//OK[0,[],0,7]");
            }
            Err(_) => {}
        };
    }

    #[test]
    fn login_should_connect() {
        let baca = make_instance();
        let req = Request::new(&baca);
        let response = req.login();

        // todo: handle tsl error
        check_response(response);
    }

    #[test]
    fn details_should_connect() {
        let baca = make_instance();
        let req = Request::new(&baca);
        let response = req.details("1");

        check_response(response);
    }

    #[test]
    fn results_should_connect() {
        let baca = make_instance();
        let req = Request::new(&baca);
        let response = req.results();

        check_response(response);
    }

    #[test]
    fn tasks_should_connect() {
        let baca = make_instance();
        let req = Request::new(&baca);
        let response = req.tasks();

        if response.is_ok() {
            let response = response.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
