use crate::workspace::InstanceData;
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{CONTENT_TYPE, COOKIE};

pub fn get_cookie(instance: &InstanceData) -> String {
    let login_response = Request::new(instance).login().unwrap();

    for (name, val) in login_response.headers() {
        tracing::debug!("Resp header: {} = {:?}", name, val);
    }

    let cookie = login_response.cookies().next().unwrap();
    tracing::debug!("got cookie {} = {}", cookie.name(), cookie.value());
    cookie.value().to_string()
}

pub fn get_submit_details(instance: &InstanceData, submit_id: &str) -> String {
    let resp = Request::new(instance).details(submit_id).unwrap();
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw submit: {}", resp);

    resp
}

struct Request<'a> {
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

pub enum RequestType {
    // Results,
    SubmitDetails(String),
    Login(String, String),
}

impl RequestType {
    pub fn payload_template(&self) -> String {
        match self {
            // RequestType::Results =>
            //     "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getAllSubmits|Z|1|2|3|4|1|5|1|".to_string(),
            RequestType::SubmitDetails(id) =>
                "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getSubmitDetails|I|1|2|3|4|1|5|".to_string() + id + "|",
            RequestType::Login(_, _) =>
                "7|0|7|{}|620F3CE7784C04B839FC8E10C6C4A753|testerka.gwt.client.acess.PrivilegesService|login|java.lang.String/2004016611|{}|{}|1|2|3|4|2|5|5|6|7|".to_string(),
        }
    }

    fn mapping(&self) -> String {
        match *self {
            // RequestType::Results => "submits".to_string(),
            RequestType::SubmitDetails(_) => "submits".to_string(),
            RequestType::Login(_, _) => "privileges".to_string(),
        }
    }
}
