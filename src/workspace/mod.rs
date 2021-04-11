use crate::baca;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::DirBuilder;

const BACA_DIR: &str = ".baca";
const INSTANCE_PATH: &str = ".baca/instance";

pub fn initialize() -> Result<(), String> {
    let baca_dir = fs::read_dir(BACA_DIR);

    if baca_dir.is_ok() {
        return Err("BaCa directory already exists.".to_string());
    }

    if let Err(e) = DirBuilder::new().create(BACA_DIR) {
        return Err(format!("Error creating BaCa directory: {}", e.to_string()));
    }

    tracing::info!("BaCa directory created successfully.");
    Ok(())
}

pub fn save(instance: &InstanceData) {
    let serialized = serde_json::to_string(instance).unwrap();
    tracing::debug!("serialized = {}", serialized);

    fs::write(INSTANCE_PATH, serialized).expect("Unable to write file");
}

// todo: uniform error reporting
pub fn read() -> InstanceData {
    let serialized = fs::read_to_string(INSTANCE_PATH).expect("Unable to read file");
    tracing::debug!("serialized = {}", serialized);

    let deserialized: InstanceData = serde_json::from_str(&serialized).unwrap();
    tracing::debug!("deserialized = {:?}", deserialized);

    deserialized
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
