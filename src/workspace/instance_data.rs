use crate::baca;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceData {
    pub host: String,
    pub login: String,
    pub password: String,
    pub permutation: String,
    pub cookie: String,
}

impl InstanceData {
    pub fn credentials(&self) -> (String, String) {
        (self.login.clone(), self.password.clone())
    }

    pub fn make_url(&self) -> String {
        format!("https://{}/{}", baca::details::SERVER_URL, self.host)
    }

    pub fn make_module_base(&self) -> String {
        format!("{}/testerka_gwt/", self.make_url())
    }

    pub fn make_payload(&self, req_type: &baca::api::RequestType) -> String {
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
