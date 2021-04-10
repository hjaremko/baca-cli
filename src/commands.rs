use crate::baca::{InstanceData, RequestBuilder};
use crate::persistence;
use crate::submit_parser::SubmitParser;
use std::rc::Rc;

pub fn init(host: &str, permutation: &str, cookie: &str) {
    let instance = Rc::new(InstanceData {
        name: host.to_string(),
        permutation: permutation.to_string(),
        cookie: cookie.to_string(),
    });

    if let Err(e) = persistence::init_repository() {
        println!("Initializing error: {}", e);
        return; // todo: return error code or something
    }

    persistence::save_baca_info(instance.as_ref());
}

pub fn submit_details(submit_id: &str) {
    let instance = Rc::from(persistence::read_baca_info());
    let req = RequestBuilder::new(instance.clone());
    let submit_resp = req.send_submit_details(submit_id);
    let raw_submit_data = submit_resp.text().expect("Invalid submit data");
    tracing::trace!("{}", raw_submit_data);

    let submit =
        SubmitParser::parse(submit_id, &instance, &raw_submit_data).expect("Error parsing submit");
    submit.print();
}
