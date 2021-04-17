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
        format!("JSESSIONID={};", self.cookie)
    }
}
