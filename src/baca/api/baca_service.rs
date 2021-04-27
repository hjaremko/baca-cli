use crate::error::Result;
use crate::model::Task;
use crate::workspace::InstanceData;

use crate::baca::api::*;
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
        get_cookie(instance)
    }

    fn get_submit_details(instance: &InstanceData, submit_id: &str) -> Result<String> {
        get_submit_details(instance, submit_id)
    }

    fn get_results(instance: &InstanceData) -> Result<String> {
        get_results(instance)
    }

    fn get_tasks(instance: &InstanceData) -> Result<String> {
        get_tasks(instance)
    }

    fn submit(instance: &InstanceData, task: &Task, file_path: &str) -> Result<()> {
        submit(instance, task, file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca;

    fn make_correct_baca() -> InstanceData {
        InstanceData {
            host: "mn2020".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: baca::details::permutation(),
            cookie: "".to_string(),
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
        match result {
            Ok(_) => panic!("Should fail!"),
            Err(e) => match e {
                Error::ProtocolError => {}
                e => assert!(matches!(e, Error::InvalidHost)),
            },
        }
    }

    fn check_invalid_login(result: Result<String>) {
        match result {
            Ok(_) => panic!("Should fail!"),
            Err(e) => match e {
                Error::ProtocolError => {}
                e => assert!(matches!(e, Error::InvalidLoginOrPassword)),
            },
        }
    }

    #[test]
    fn get_cookie_on_correct_host_should_fail_login() {
        let baca = make_correct_baca();
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
        let baca = make_correct_baca();
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
}
