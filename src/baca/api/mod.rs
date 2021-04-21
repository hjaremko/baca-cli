mod request;
mod request_type;

pub use self::request::Request;
pub use self::request_type::RequestType;

use crate::baca::details::EMPTY_RESPONSE;
use crate::error::Error;
use crate::error::Result;
use crate::model::Task;
use crate::workspace::InstanceData;
use reqwest::blocking::multipart;
use reqwest::header::COOKIE;

pub fn get_cookie(instance: &InstanceData) -> Result<String> {
    let login_response = Request::new(instance).login()?;

    for (name, val) in login_response.headers() {
        tracing::debug!("Resp header: {} = {:?}", name, val);
    }

    let cookie = login_response
        .cookies()
        .next()
        .expect("No cookie in response!");
    tracing::debug!("Cookie: {} = {}", cookie.name(), cookie.value());
    Ok(cookie.value().to_string())
}

pub fn get_submit_details(instance: &InstanceData, submit_id: &str) -> Result<String> {
    let resp = Request::new(instance).details(submit_id)?;
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw submit: {}", resp);

    if resp == EMPTY_RESPONSE || resp.contains("failed") {
        return Err(Error::InvalidSubmitId);
    }

    check_for_empty_response(resp)
}

pub fn get_results(instance: &InstanceData) -> Result<String> {
    let resp = Request::new(instance).results()?;
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw results: {}", resp);

    check_for_empty_response(resp)
}

pub fn get_tasks(instance: &InstanceData) -> Result<String> {
    let resp = Request::new(instance).tasks()?;
    let resp = resp.text().expect("Invalid submit data");
    tracing::debug!("Received raw tasks: {}", resp);

    check_for_empty_response(resp)
}

pub fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<()> {
    tracing::debug!("{:?}", task);
    let form = multipart::Form::new()
        .text("zadanie", task.id.clone())
        .text("jezyk", task.language.code())
        .file("zrodla", file_path)
        .map_err(|e| Error::ReadingSourceError(e.into()))?;

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = format!("https://baca.ii.uj.edu.pl/{}/sendSubmit", instance.host);
    tracing::debug!("SendSubmit url: {}", url);

    let resp = client
        .post(url)
        .multipart(form)
        .header(COOKIE, instance.make_cookie())
        .send()?;

    let resp = resp.text().expect("Invalid response.");
    tracing::debug!("Response: {}", resp);

    match resp.as_str() {
        "Niezalogowany jesteś" => Err(Error::LoggedOutError),
        "Błąd" => Err(Error::SubmitError),
        _ => Ok(()),
    }
}

fn check_for_empty_response(resp: String) -> Result<String> {
    if resp == EMPTY_RESPONSE {
        Err(Error::LoggedOutError)
    } else {
        Ok(resp)
    }
}
