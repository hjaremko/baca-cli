use crate::baca::{InstanceData, RequestBuilder};
use crate::persistence;
use crate::submit_parser::SubmitParser;
use std::borrow::{Borrow, BorrowMut};

pub fn init(host: &str, login: &str, pass: &str) {
    let mut instance = InstanceData {
        host: host.to_string(),
        login: login.to_string(),
        password: pass.to_string(),
        permutation: "5A4AE95C27260DF45F17F9BF027335F6".to_string(),
        cookie: "".to_string(),
    };

    if let Err(e) = persistence::init_repository() {
        println!("Initializing error: {}", e);
        return; // todo: return error code or something
    }

    let req = RequestBuilder::new(instance.clone());
    let login_response = req.send_login();

    for (name, val) in login_response.headers() {
        tracing::debug!("Resp header: {} = {:?}", name, val);
    }

    for cookie in login_response.cookies() {
        tracing::debug!("got cookie {} = {}", cookie.name(), cookie.value());
        instance.borrow_mut().cookie = cookie.value().to_string();
    }

    persistence::save_baca_info(instance.borrow());
}

pub fn submit_details(submit_id: &str) {
    let instance = persistence::read_baca_info();
    let req = RequestBuilder::new(instance.clone());
    let submit_resp = req.send_submit_details(submit_id);
    let raw_submit_data = submit_resp.text().expect("Invalid submit data");

    tracing::debug!("Received: {}", raw_submit_data);

    let submit = SubmitParser::parse(submit_id, &*instance.borrow(), &raw_submit_data)
        .expect("Error parsing submit");
    submit.print();
}
