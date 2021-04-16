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

    fn make_request(&self, req_type: RequestType) -> RequestBuilder {
        let post_url = format!("{}{}", self.instance.make_module_base(), req_type.mapping());
        let payload = self.instance.make_payload(&req_type);

        tracing::debug!("Making request to: {}", post_url);
        tracing::debug!("Making request with payload: {}", payload);

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
