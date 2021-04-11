use crate::model::Submit;
use crate::{baca, workspace};

pub fn init(host: &str, login: &str, pass: &str) {
    let mut instance = workspace::InstanceData {
        host: host.to_string(),
        login: login.to_string(),
        password: pass.to_string(),
        permutation: baca::details::permutation(),
        cookie: "".to_string(),
    };

    if let Err(e) = workspace::initialize() {
        println!("Initializing error: {}", e);
        return; // todo: return error code or something
    }

    instance.cookie = baca::api::get_cookie(&instance);
    workspace::save(&instance);
}

pub fn details(submit_id: &str) {
    let instance = workspace::read();
    let submit = baca::api::get_submit_details(&instance, submit_id);
    let submit = Submit::parse(submit_id, &instance, &submit).expect("Error parsing submit");

    submit.print();
}

pub fn refresh() {
    let mut instance = workspace::read();
    instance.cookie = baca::api::get_cookie(&instance);
    workspace::save(&instance);
}
