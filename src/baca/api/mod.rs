mod request;
mod request_type;

pub use self::request::Request;
pub use self::request_type::RequestType;

use crate::baca::details::EMPTY_RESPONSE;
use crate::error::Error;
use crate::error::Result;
use crate::model::Task;
use crate::workspace::InstanceData;
use reqwest::blocking::{multipart, Response};
use reqwest::header::COOKIE;
use tracing::debug;

pub mod baca_service;

pub fn get_cookie(instance: &InstanceData) -> Result<String> {
    let login_response = Request::new(instance).login()?;
    log_response_details(&login_response);
    check_response_status(&login_response)?;
    extract_cookie(&login_response)
}

fn log_response_details(login_response: &Response) {
    for (name, val) in login_response.headers() {
        debug!("Response header: {} = {:?}", name, val);
    }

    debug!("Status code: {}", login_response.status());
}

fn extract_cookie(response: &Response) -> Result<String> {
    let cookie = response
        .cookies()
        .next()
        .ok_or(Error::InvalidLoginOrPassword)?;

    debug!("Cookie: {} = {}", cookie.name(), cookie.value());
    Ok(cookie.value().to_string())
}

pub fn get_submit_details(instance: &InstanceData, submit_id: &str) -> Result<String> {
    let resp = Request::new(instance).details(submit_id)?;
    let resp = resp.text().expect("Invalid submit data");
    debug!("Received raw submit: {}", resp);

    if resp == EMPTY_RESPONSE || resp.contains("failed") {
        return Err(Error::InvalidSubmitId);
    }

    check_for_empty_response(resp)
}

pub fn get_results(instance: &InstanceData) -> Result<String> {
    let resp = Request::new(instance).results()?;
    let resp = resp.text().expect("Invalid submit data");
    debug!("Received raw results: {}", resp);

    check_for_empty_response(resp)
}

pub fn get_tasks(instance: &InstanceData) -> Result<String> {
    let resp = Request::new(instance).tasks()?;
    check_response_status(&resp)?;

    let resp = resp.text().expect("Invalid submit data");
    debug!("Received raw tasks: {}", resp);

    check_for_empty_response(resp)
}

pub fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<()> {
    debug!("{:?}", task);
    let form = multipart::Form::new()
        .text("zadanie", task.id.clone())
        .text("jezyk", task.language.code())
        .file("zrodla", file_path)
        .map_err(|e| Error::ReadingSourceError(e.into()))?;

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = format!("https://baca.ii.uj.edu.pl/{}/sendSubmit", instance.host);
    debug!("SendSubmit url: {}", url);

    let resp = client
        .post(url)
        .multipart(form)
        .header(COOKIE, instance.make_cookie())
        .send()?;

    let resp = resp.text().expect("Invalid response.");
    debug!("Response: {}", resp);

    match resp.as_str() {
        "Niezalogowany jesteś" => Err(Error::LoggedOutError),
        "Błąd" => Err(Error::SubmitError),
        _ => Ok(()),
    }
}

fn check_response_status(response: &Response) -> Result<()> {
    if response.status().as_str() == "404" {
        return Err(Error::InvalidHost);
    };

    Ok(())
}

fn check_for_empty_response(resp: String) -> Result<String> {
    if resp == EMPTY_RESPONSE {
        Err(Error::LoggedOutError)
    } else {
        Ok(resp)
    }
}
