use crate::baca::api::baca_api::BacaApi;
use crate::baca::api::Request;
use crate::baca::details::EMPTY_RESPONSE;
use crate::error::{Error, Result};
use crate::model::{Results, Submit, Task, Tasks};
use crate::parse::from_baca_output::FromBacaOutput;
use crate::workspace::InstanceData;
use reqwest::blocking::Response;
use std::str::FromStr;
use tracing::debug;

pub struct BacaService {}

impl Default for BacaService {
    fn default() -> Self {
        Self {}
    }
}

impl BacaApi for BacaService {
    fn get_cookie(&self, instance: &InstanceData) -> Result<String> {
        let login_response = Request::new(instance).login()?;
        log_response_details(&login_response);
        check_response_status(&login_response)?;
        extract_cookie(&login_response)
    }

    fn get_submit_details(&self, instance: &InstanceData, submit_id: &str) -> Result<Submit> {
        let resp = Request::new(instance).details(submit_id)?;
        check_response_status(&resp)?;
        let resp = resp.text()?;
        debug!("Received raw submit: {}", resp);

        if resp.contains("failed") {
            return Err(Error::InvalidSubmitId);
        }

        Ok(Submit::parse(instance, &check_for_empty_response(resp)?))
    }

    fn get_results(&self, instance: &InstanceData) -> Result<Results> {
        let resp = Request::new(instance).results()?;
        check_response_status(&resp)?;
        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw results: {}", resp);

        Ok(Results::from_baca_output(
            instance,
            &check_for_empty_response(resp)?,
        ))
    }

    fn get_tasks(&self, instance: &InstanceData) -> Result<Tasks> {
        let resp = Request::new(instance).tasks()?;
        check_response_status(&resp)?;

        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw tasks: {}", resp);

        Tasks::from_str(&check_for_empty_response(resp)?)
    }

    fn submit(&self, instance: &InstanceData, task: &Task, file_path: &str) -> Result<()> {
        debug!("{:?}", task);
        let resp = Request::new(instance).submit(task, file_path)?;
        let resp = resp.text()?;
        debug!("Response: {}", resp);

        match resp.as_str() {
            "Niezalogowany jesteś" => Err(Error::LoggedOut),
            "Błąd" => Err(Error::Submit),
            _ => Ok(()),
        }
    }
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

fn check_response_status(response: &Response) -> Result<()> {
    if response.status().as_str() == "404" {
        return Err(Error::InvalidHost);
    };

    Ok(())
}

fn check_for_empty_response(resp: String) -> Result<String> {
    if resp == EMPTY_RESPONSE {
        Err(Error::LoggedOut)
    } else {
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca;
    use crate::baca::details::language::Language::Unsupported;
    use std::fmt::Debug;

    fn make_correct_baca_invalid_session() -> InstanceData {
        InstanceData {
            host: "mn2020".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: baca::details::permutation(),
            cookie: "invalid".to_string(),
        }
    }

    fn make_incorrect_baca() -> InstanceData {
        InstanceData {
            host: "invalid".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: "invalid".to_string(),
            cookie: "".to_string(),
        }
    }

    fn check_invalid_host<T>(result: Result<T>)
    where
        T: Debug,
    {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::InvalidHost));
    }

    fn check_invalid_login<T>(result: Result<T>)
    where
        T: Debug,
    {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::InvalidLoginOrPassword));
    }

    fn check_logged_out<T>(result: Result<T>)
    where
        T: Debug,
    {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::LoggedOut));
    }

    #[test]
    fn get_cookie_on_correct_host_should_fail_login() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_cookie(&baca);

        check_invalid_login(result)
    }

    #[test]
    fn get_cookie_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_cookie(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_task_on_correct_host_should_succeed() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let actual = api.get_tasks(&baca).unwrap();
        let expected = Tasks {
            tasks: vec![
                Task::new("1", Unsupported, "[A] Zera funkcji", 69),
                Task::new("2", Unsupported, "[B] Metoda Newtona", 58),
                Task::new(
                    "3",
                    Unsupported,
                    "[C] FAD\\x3Csup\\x3E2\\x3C/sup\\x3E - Pochodne mieszane",
                    62,
                ),
                Task::new("4", Unsupported, "[D] Skalowany Gauss", 52),
                Task::new("5", Unsupported, "[E] Metoda SOR", 64),
                Task::new("6", Unsupported, "[F] Interpolacja", 63),
                Task::new("7", Unsupported, "[G] Funkcje sklejane", 59),
                Task::new("8", Unsupported, "A2", 1),
                Task::new("9", Unsupported, "B2", 2),
                Task::new("10", Unsupported, "C2", 1),
                Task::new("11", Unsupported, "D2", 2),
                Task::new("12", Unsupported, "E2", 1),
                Task::new("13", Unsupported, "F2", 3),
                Task::new("14", Unsupported, "G2", 2),
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_task_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_tasks(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_details_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_submit_details(&baca, "123");

        check_invalid_host(result);
    }

    #[test]
    fn get_details_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_submit_details(&baca, "123");

        check_logged_out(result);
    }

    #[test]
    fn get_results_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_results(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_results_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_results(&baca);

        check_logged_out(result);
    }
}
