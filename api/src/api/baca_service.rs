// use crate::api::baca_api::BacaApi;
use crate::api::details::EMPTY_RESPONSE;
use crate::api::Request;
use crate::error::{Error, Result};
use crate::model::{Results, Submit, Task, Tasks};
// use crate::parse::from_baca_output::FromBacaOutput;
use reqwest::blocking::Response;
use std::str::FromStr;
use tracing::{debug, info};
use crate::api::baca_api::BacaApi;
use crate::model::Language;
use crate::network::ConnectionConfig;
use crate::parse::from_baca_output::FromBacaOutput;

#[derive(Default)]
pub struct BacaService {}

impl BacaApi for BacaService {
    fn get_cookie(&self, connection_config: &ConnectionConfig) -> Result<String> {
        let login_response = Request::new(connection_config).login()?;
        log_response_details(&login_response);
        check_response_status(&login_response)?;
        extract_cookie(&login_response)
    }

    fn get_submit_details(
        &self,
        connection_config: &ConnectionConfig,
        submit_id: &str,
    ) -> Result<Submit> {
        let resp = Request::new(connection_config).details(submit_id)?;
        check_response_status(&resp)?;
        let resp = resp.text()?;
        debug!("Received raw submit: {}", resp);

        if resp.contains("failed") {
            return Err(Error::InvalidSubmitId);
        }

        Ok(Submit::parse(
            connection_config,
            &check_for_empty_response(resp)?,
        ))
    }

    fn get_results(&self, connection_config: &ConnectionConfig) -> Result<Results> {
        let resp = Request::new(connection_config).results()?;
        check_response_status(&resp)?;
        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw results: {}", resp);

        Ok(Results::from_baca_output(
            connection_config,
            &check_for_empty_response(resp)?,
        ))
    }

    fn get_results_by_task(
        &self,
        connection_config: &ConnectionConfig,
        task_id: &str,
    ) -> Result<Results> {
        let tasks = self.get_tasks(connection_config)?;
        let task = tasks.get_by_id(task_id)?;
        info!("Showing logs for task {}", &task.problem_name);
        Ok(self
            .get_results(connection_config)?
            .filter_by_task(&task.problem_name))
    }

    fn get_tasks(&self, connection_config: &ConnectionConfig) -> Result<Tasks> {
        let resp = Request::new(connection_config).tasks()?;
        check_response_status(&resp)?;

        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw tasks: {}", resp);

        Tasks::from_str(&check_for_empty_response(resp)?)
    }

    fn submit(
        &self,
        connection_config: &ConnectionConfig,
        task: &Task,
        file_path: &str,
    ) -> Result<()> {
        debug!("{:?}", task);
        let resp = Request::new(connection_config).submit(task, file_path)?;
        let resp = resp.text()?;
        debug!("Response: {}", resp);

        match resp.as_str() {
            "Niezalogowany jesteś" => Err(Error::LoggedOut),
            "Błąd" => Err(Error::TaskNotActive),
            _ => Ok(()),
        }
    }

    fn get_allowed_language(
        &self,
        connection_config: &ConnectionConfig,
        task_id: &str,
    ) -> Result<Option<Language>> {
        let response = Request::new(connection_config).allowed_languages(task_id)?;
        check_response_status(&response)?;
        let response = response.text()?;
        debug!("Received raw allowed languages: {:?}", response);
        Ok(Option::<Language>::from_baca_output(
            connection_config,
            &response,
        ))
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
    use crate::api;
    use crate::model::Language::Unsupported;
    use std::fmt::Debug;

    fn make_correct_baca_invalid_session() -> ConnectionConfig {
        ConnectionConfig {
            host: "mn2020".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: api::details::permutation(),
            cookie: "invalid".to_string(),
        }
    }

    fn make_incorrect_baca() -> ConnectionConfig {
        ConnectionConfig {
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
    #[ignore]
    fn get_cookie_on_correct_host_should_fail_login() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_cookie(&baca);

        check_invalid_login(result)
    }

    #[test]
    #[ignore]
    fn get_cookie_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_cookie(&baca);

        check_invalid_host(result);
    }

    #[test]
    #[ignore]
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
    #[ignore]
    fn get_task_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_tasks(&baca);

        check_invalid_host(result);
    }

    #[test]
    #[ignore]
    fn get_details_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_submit_details(&baca, "123");

        check_invalid_host(result);
    }

    #[test]
    #[ignore]
    fn get_details_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_submit_details(&baca, "123");

        check_logged_out(result);
    }

    #[test]
    #[ignore]
    fn get_results_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_results(&baca);

        check_invalid_host(result);
    }

    #[test]
    #[ignore]
    fn get_results_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_results(&baca);

        check_logged_out(result);
    }

    #[test]
    #[ignore]
    fn get_languages_expired_task_should_return_empty() {
        let connection = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_allowed_language(&connection, "1").unwrap();

        assert!(result.is_none());
    }

    #[test]
    #[ignore]
    fn get_languages_on_incorrect_host_should_fail() {
        let connection = make_incorrect_baca();
        let api = BacaService::default();
        let result = api.get_allowed_language(&connection, "1");

        check_invalid_host(result);
    }

    #[test]
    #[ignore]
    fn get_languages_on_incorrect_task_should_return_empty() {
        let connection = make_correct_baca_invalid_session();
        let api = BacaService::default();
        let result = api.get_allowed_language(&connection, "12323").unwrap();

        assert!(result.is_none());
    }
}
