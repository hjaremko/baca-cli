use crate::workspace::InstanceData;

mod request;
mod request_type;
pub use self::request::Request;
pub use self::request_type::RequestType;

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
    tracing::debug!("Received raw submit: {}", resp); // todo: handle //OK[0,[],0,7]

    resp
}

pub fn get_results(instance: &InstanceData) -> String {
    let resp = Request::new(instance).results().unwrap();
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw results: {}", resp); // todo: handle //OK[0,[],0,7]

    resp
}
