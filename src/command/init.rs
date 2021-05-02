use crate::baca::api::baca_service::BacaApi;
use crate::command::Command;
use crate::workspace::Workspace;
use crate::{baca, error, workspace};
use clap::ArgMatches;
use tracing::{debug, info};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
trait Prompt {
    fn interact(&self) -> error::Result<String>;
}

struct Input;

impl Prompt for Input {
    fn interact(&self) -> error::Result<String> {
        Ok(dialoguer::Input::<String>::new()
            .with_prompt("Login")
            .interact()?)
    }
}

struct Password;

impl Prompt for Password {
    fn interact(&self) -> error::Result<String> {
        Ok(dialoguer::Password::new()
            .with_prompt("Password")
            .interact()?)
    }
}

pub struct Init {
    host: String,
    login: Option<String>,
    password: Option<String>,
    login_prompt: Box<dyn Prompt>,
    password_prompt: Box<dyn Prompt>,
}

impl From<&ArgMatches<'_>> for Init {
    fn from(args: &ArgMatches) -> Self {
        let host = args.value_of("host").unwrap();
        let login = args.value_of("login").map(|x| x.to_string());
        let password = args.value_of("password").map(|x| x.to_string());

        Self {
            host: host.to_string(),
            login,
            password,
            login_prompt: Box::new(Input {}),
            password_prompt: Box::new(Password {}),
        }
    }
}

impl Init {
    fn get_login(&self) -> error::Result<String> {
        let login = self.login.clone();

        if let Some(login) = login {
            return Ok(login);
        }

        self.login_prompt.interact()
    }

    fn get_password(&self) -> error::Result<String> {
        let pass = self.password.clone();

        if let Some(pass) = pass {
            return Ok(pass);
        }

        self.password_prompt.interact()
    }
}

impl Command for Init {
    // todo: -r to override current config
    fn execute<W: Workspace, A: BacaApi>(self) -> error::Result<()> {
        info!("Initializing Baca workspace.");

        let login = self.get_login()?;
        let password = self.get_password()?;

        debug!("Host: {}", self.host);
        debug!("Login: {}", login);
        debug!("Password: {}", password);

        let mut instance = workspace::InstanceData {
            host: self.host,
            login,
            password,
            permutation: baca::details::permutation(),
            cookie: "".to_string(),
        };

        let cleanup_directory = |e| match e {
            error::Error::WorkspaceAlreadyInitialized => e,
            _ => {
                W::remove_workspace().expect("Cannot cleanup baca directory");
                e
            }
        };

        W::initialize().map_err(cleanup_directory)?;
        instance.cookie = A::get_cookie(&instance).map_err(cleanup_directory)?;
        W::save_instance(&instance).map_err(cleanup_directory)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_service::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    // todo: tests::utils
    fn make_mock_instance() -> InstanceData {
        InstanceData {
            host: "host".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: baca::details::permutation(),
            cookie: "".to_string(),
        }
    }

    #[test]
    #[serial]
    fn success_test() {
        let ctx_init = MockWorkspace::initialize_context();
        ctx_init.expect().once().returning(|| Ok(()));

        let ctx_save = MockWorkspace::save_instance_context();
        ctx_save
            .expect()
            .withf(|x| {
                let mut expected = make_mock_instance();
                expected.cookie = "ok_cookie".to_string();

                *x == expected
            })
            .returning(|_| Ok(()));

        let ctx_api = MockBacaApi::get_cookie_context();
        ctx_api
            .expect()
            .withf(|x| *x == make_mock_instance())
            .returning(|_| Ok("ok_cookie".to_string()));

        let init = Init {
            host: "host".to_string(),
            login: Some("login".to_string()),
            password: Some("pass".to_string()),
            login_prompt: Box::new(Input {}),
            password_prompt: Box::new(Password {}),
        };
        let result = init.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }

    #[test]
    #[serial]
    fn no_provided_login_should_invoke_prompt() {
        let mut input_prompt_mock = MockPrompt::new();
        input_prompt_mock
            .expect_interact()
            .once()
            .returning(|| Ok("prompt_login".to_string()));

        let ctx_init = MockWorkspace::initialize_context();
        ctx_init.expect().once().returning(|| Ok(()));

        let ctx_save = MockWorkspace::save_instance_context();
        ctx_save
            .expect()
            .withf(|x| {
                *x == InstanceData {
                    host: "host".to_string(),
                    login: "prompt_login".to_string(),
                    password: "pass".to_string(),
                    permutation: baca::details::permutation(),
                    cookie: "ok_cookie".to_string(),
                }
            })
            .returning(|_| Ok(()));

        let ctx_api = MockBacaApi::get_cookie_context();
        ctx_api.expect().returning(|_| Ok("ok_cookie".to_string()));

        let init = Init {
            host: "host".to_string(),
            login: None,
            password: Some("pass".to_string()),
            login_prompt: Box::new(input_prompt_mock),
            password_prompt: Box::new(Password {}),
        };

        let result = init.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }

    #[test]
    #[serial]
    fn no_provided_password_should_invoke_prompt() {
        let mut input_prompt_mock = MockPrompt::new();
        input_prompt_mock
            .expect_interact()
            .once()
            .returning(|| Ok("prompt_password".to_string()));

        let ctx_init = MockWorkspace::initialize_context();
        ctx_init.expect().once().returning(|| Ok(()));

        let ctx_save = MockWorkspace::save_instance_context();
        ctx_save
            .expect()
            .withf(|x| {
                *x == InstanceData {
                    host: "host".to_string(),
                    login: "login".to_string(),
                    password: "prompt_password".to_string(),
                    permutation: baca::details::permutation(),
                    cookie: "ok_cookie".to_string(),
                }
            })
            .returning(|_| Ok(()));

        let ctx_api = MockBacaApi::get_cookie_context();
        ctx_api.expect().returning(|_| Ok("ok_cookie".to_string()));

        let init = Init {
            host: "host".to_string(),
            login: Some("login".to_string()),
            password: None,
            login_prompt: Box::new(Input {}),
            password_prompt: Box::new(input_prompt_mock),
        };

        let result = init.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }

    #[test]
    #[serial]
    fn no_provided_login_and_password_should_invoke_prompt() {
        let mut input_prompt_mock = MockPrompt::new();
        input_prompt_mock
            .expect_interact()
            .once()
            .returning(|| Ok("prompt_login".to_string()));

        let mut password_prompt_mock = MockPrompt::new();
        password_prompt_mock
            .expect_interact()
            .once()
            .returning(|| Ok("prompt_password".to_string()));

        let ctx_init = MockWorkspace::initialize_context();
        ctx_init.expect().once().returning(|| Ok(()));

        let ctx_save = MockWorkspace::save_instance_context();
        ctx_save
            .expect()
            .withf(|x| {
                *x == InstanceData {
                    host: "host".to_string(),
                    login: "prompt_login".to_string(),
                    password: "prompt_password".to_string(),
                    permutation: baca::details::permutation(),
                    cookie: "ok_cookie".to_string(),
                }
            })
            .returning(|_| Ok(()));

        let ctx_api = MockBacaApi::get_cookie_context();
        ctx_api.expect().returning(|_| Ok("ok_cookie".to_string()));

        let init = Init {
            host: "host".to_string(),
            login: None,
            password: None,
            login_prompt: Box::new(input_prompt_mock),
            password_prompt: Box::new(password_prompt_mock),
        };

        let result = init.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }
}

//todo: fail cases
//todo: refactor tests
