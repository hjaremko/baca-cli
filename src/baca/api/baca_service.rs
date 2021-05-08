use crate::baca::api::Request;
use crate::baca::details::EMPTY_RESPONSE;
use crate::error::{Error, Result};
use crate::model::Task;
use crate::workspace::InstanceData;
use reqwest::blocking::Response;
use tracing::debug;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait BacaApi {
    fn get_cookie(instance: &InstanceData) -> Result<String>;
    fn get_submit_details(instance: &InstanceData, submit_id: &str) -> Result<String>;
    fn get_results(instance: &InstanceData) -> Result<String>;
    fn get_tasks(instance: &InstanceData) -> Result<String>;
    fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<()>;
}

pub struct BacaService {}

impl BacaApi for BacaService {
    fn get_cookie(instance: &InstanceData) -> Result<String> {
        let login_response = Request::new(instance).login()?;
        log_response_details(&login_response);
        check_response_status(&login_response)?;
        extract_cookie(&login_response)
    }

    fn get_submit_details(instance: &InstanceData, submit_id: &str) -> Result<String> {
        let resp = Request::new(instance).details(submit_id)?;
        check_response_status(&resp)?;
        let resp = resp.text()?;
        debug!("Received raw submit: {}", resp);

        if resp.contains("failed") {
            return Err(Error::InvalidSubmitId);
        }

        check_for_empty_response(resp)
    }

    fn get_results(instance: &InstanceData) -> Result<String> {
        let resp = Request::new(instance).results()?;
        check_response_status(&resp)?;
        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw results: {}", resp);

        check_for_empty_response(resp)
    }

    fn get_tasks(instance: &InstanceData) -> Result<String> {
        let resp = Request::new(instance).tasks()?;
        check_response_status(&resp)?;

        let resp = resp.text().expect("Invalid submit data");
        debug!("Received raw tasks: {}", resp);

        check_for_empty_response(resp)
    }

    fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<()> {
        debug!("{:?}", task);
        let resp = Request::new(instance).submit(task, file_path)?;
        let resp = resp.text()?;
        debug!("Response: {}", resp);

        match resp.as_str() {
            "Niezalogowany jesteś" => Err(Error::LoggedOutError),
            "Błąd" => Err(Error::SubmitError),
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
        Err(Error::LoggedOutError)
    } else {
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca;

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

    fn check_result(result: Result<String>, expected: &str) {
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(e) => match e {
                Error::ProtocolError => {}
                _ => panic!("Should not fail!"),
            },
        }
    }

    fn check_invalid_host(result: Result<String>) {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::InvalidHost));
    }

    fn check_invalid_login(result: Result<String>) {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::InvalidLoginOrPassword));
    }

    fn check_logged_out(result: Result<String>) {
        let e = result.expect_err("Should fail");
        assert!(matches!(e, Error::LoggedOutError));
    }

    #[test]
    fn get_cookie_on_correct_host_should_fail_login() {
        let baca = make_correct_baca_invalid_session();
        let result = BacaService::get_cookie(&baca);

        check_invalid_login(result)
    }

    #[test]
    fn get_cookie_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let result = BacaService::get_cookie(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_task_on_correct_host_should_succeed() {
        let baca = make_correct_baca_invalid_session();
        let result = BacaService::get_tasks(&baca);

        let expected = r#"//OK[0,41,40,39,3,3,7,38,37,3,3,10,36,35,3,3,4,34,33,3,3,7,32,31,3,3,4,30,29,3,3,7,28,27,3,3,4,26,25,3,3,24,23,22,3,3,21,20,19,3,3,18,17,16,3,3,15,14,13,3,3,12,11,10,3,3,9,8,7,3,3,6,5,4,3,3,14,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","1","[A] Zera funkcji","69","2","[B] Metoda Newtona","58","3","[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane","62","4","[D] Skalowany Gauss","52","5","[E] Metoda SOR","64","6","[F] Interpolacja","63","7","[G] Funkcje sklejane","59","8","A2","9","B2","10","C2","11","D2","12","E2","13","F2","14","G2","id","nazwa","liczba OK"],0,7]"#;
        check_result(result, expected);
    }

    #[test]
    fn get_task_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let result = BacaService::get_tasks(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_details_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let result = BacaService::get_submit_details(&baca, "123");

        check_invalid_host(result);
    }

    #[test]
    fn get_details_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let result = BacaService::get_submit_details(&baca, "123");

        check_logged_out(result);
    }

    #[test]
    fn get_results_on_incorrect_host_should_fail() {
        let baca = make_incorrect_baca();
        let result = BacaService::get_results(&baca);

        check_invalid_host(result);
    }

    #[test]
    fn get_results_on_incorrect_session_should_fail() {
        let baca = make_correct_baca_invalid_session();
        let result = BacaService::get_results(&baca);

        check_logged_out(result);
    }
}
