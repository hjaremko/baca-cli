use crate::workspace::InstanceData;

mod request;
mod request_type;
pub use self::request::Request;
pub use self::request_type::RequestType;
use crate::model::Task;
use reqwest::blocking::multipart;
use reqwest::header::COOKIE;

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

pub fn get_tasks(instance: &InstanceData) -> String {
    let resp = Request::new(instance).tasks().unwrap();
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw tasks: {}", resp); // todo: handle //OK[0,[],0,7]

    resp
}

pub fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<(), String> {
    tracing::debug!("{:?}", task);
    let form = multipart::Form::new()
        .text("zadanie", task.id.clone())
        .text("jezyk", task.language.code())
        .file("zrodla", file_path)
        .unwrap();

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let url = format!("https://baca.ii.uj.edu.pl/{}/sendSubmit", instance.host);
    tracing::debug!("SendSubmit url: {}", url);

    let resp = client
        .post(url)
        .multipart(form)
        .header(COOKIE, instance.make_cookie())
        .send()
        .unwrap();

    let resp = resp.text().unwrap();
    tracing::debug!("Response: {}", resp);

    // todo: return Result
    match resp.as_str() {
        "Niezalogowany jesteś" => Err(
            "The session cookie has expired, type 'baca refresh' to re-log and try again."
                .to_string(),
        ),
        "Błąd" => Err("Error sending submit. Is the task still active?".to_string()),
        _ => Ok(()),
    }
}
