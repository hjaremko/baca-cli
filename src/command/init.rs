use crate::api;
use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::update::BacaRelease;
use crate::workspace::{ConfigObject, Workspace};
use crate::{error, workspace};
use clap::ArgMatches;
#[cfg(test)]
use mockall::{automock, predicate::*};
use tracing::{debug, info};

#[cfg_attr(test, automock)]
trait Prompt {
    fn interact(&self) -> error::Result<String>;
}

struct Input(&'static str);

impl Prompt for Input {
    fn interact(&self) -> error::Result<String> {
        Ok(dialoguer::Input::<String>::new()
            .with_prompt(self.0)
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
    host: Option<String>,
    login: Option<String>,
    password: Option<String>,
    login_prompt: Box<dyn Prompt>,
    password_prompt: Box<dyn Prompt>,
    host_prompt: Box<dyn Prompt>,
}

impl From<&ArgMatches<'_>> for Init {
    fn from(args: &ArgMatches) -> Self {
        let host = args.value_of("host").map(|x| x.to_string());
        let login = args.value_of("login").map(|x| x.to_string());
        let password = args.value_of("password").map(|x| x.to_string());

        Self {
            host,
            login,
            password,
            login_prompt: Box::new(Input("Login")),
            password_prompt: Box::new(Password {}),
            host_prompt: Box::new(Input("Host")),
        }
    }
}

impl Init {
    fn get_host(&self) -> error::Result<String> {
        if self.host.as_ref().is_none() {
            return self.host_prompt.interact();
        }

        Ok(self.host.as_ref().unwrap().clone())
    }

    fn get_login(&self) -> error::Result<String> {
        if self.login.as_ref().is_none() {
            return self.login_prompt.interact();
        }

        Ok(self.login.as_ref().unwrap().clone())
    }

    fn get_password(&self) -> error::Result<String> {
        if self.password.as_ref().is_none() {
            return self.password_prompt.interact();
        }

        Ok(self.password.as_ref().unwrap().clone())
    }
}

impl Command for Init {
    // todo: -r to override current config
    fn execute<W, A>(self, workspace: &W, api: &A) -> error::Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        info!("Initializing Baca workspace.");

        let host = self.get_host()?;
        let login = self.get_login()?;
        let password = self.get_password()?;

        debug!("Host: {}", host);
        debug!("Login: {}", login);
        debug!("Password: {}", password);

        let mut config = workspace::ConnectionConfig {
            host,
            login,
            password,
            permutation: api::details::permutation(),
            cookie: "".to_string(),
        };

        let cleanup_directory = |e| match e {
            error::Error::WorkspaceAlreadyInitialized => e,
            _ => {
                workspace
                    .remove_workspace()
                    .expect("Cannot cleanup baca directory");
                e
            }
        };

        workspace.initialize().map_err(cleanup_directory)?;
        config.cookie = api.get_cookie(&config).map_err(cleanup_directory)?;
        config.save_config(workspace).map_err(cleanup_directory)?;
        save_version(workspace)
    }
}

fn save_version<W: Workspace>(workspace: &W) -> error::Result<()> {
    let version = BacaRelease::new(env!("CARGO_PKG_VERSION"), "");
    version.save_config(workspace)
}

#[cfg(test)]
mod tests {
    use crate::api;
    use crate::api::baca_api::MockBacaApi;
    use crate::workspace::{ConnectionConfig, MockWorkspace};

    use super::*;

    // todo: tests::utils
    fn make_mock_connection_config() -> ConnectionConfig {
        ConnectionConfig {
            host: "host".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: api::details::permutation(),
            cookie: "".to_string(),
        }
    }

    fn make_never_called_prompt_mock() -> MockPrompt {
        let mut mock = MockPrompt::new();
        mock.expect_interact().never();
        mock
    }

    fn make_prompt_mock(return_val: &'static str) -> MockPrompt {
        let mut mock = MockPrompt::new();
        mock.expect_interact()
            .once()
            .returning(move || Ok(return_val.to_string()));
        mock
    }

    fn make_baca_config(
        host: &'static str,
        login: &'static str,
        password: &'static str,
    ) -> ConnectionConfig {
        let host = host.to_string();
        let login = login.to_string();
        let password = password.to_string();
        ConnectionConfig {
            host,
            login,
            password,
            permutation: api::details::permutation(),
            cookie: "ok_cookie".to_string(),
        }
    }

    #[test]
    fn success_test() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_initialize()
            .once()
            .returning(|| Ok(()));
        mock_workspace
            .expect_save_config_object::<BacaRelease>()
            .returning(|_| Ok(()));

        mock_workspace
            .expect_save_config_object()
            .withf(|x: &ConnectionConfig| {
                let mut expected = make_mock_connection_config();
                expected.cookie = "ok_cookie".to_string();

                *x == expected
            })
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .withf(|x| *x == make_mock_connection_config())
            .returning(|_| Ok("ok_cookie".to_string()));

        let init = Init {
            host: Some("host".to_string()),
            login: Some("login".to_string()),
            password: Some("pass".to_string()),
            login_prompt: Box::new(Input("Login")),
            password_prompt: Box::new(Password {}),
            host_prompt: Box::new(Input("Host")),
        };
        let result = init.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }

    #[test]
    fn no_provided_login_should_invoke_prompt() {
        let login_prompt_mock = make_prompt_mock("prompt_login");
        let password_prompt_mock = make_never_called_prompt_mock();
        let host_prompt_mock = make_never_called_prompt_mock();

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_initialize()
            .once()
            .returning(|| Ok(()));
        mock_workspace
            .expect_save_config_object::<BacaRelease>()
            .returning(|_| Ok(()));

        let expected_config = make_baca_config("host", "prompt_login", "pass");
        let expected_cookie = expected_config.cookie.clone();

        mock_workspace
            .expect_save_config_object()
            .withf(move |x: &ConnectionConfig| *x == expected_config)
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .returning(move |_| Ok(expected_cookie.clone()));

        let init = Init {
            host: Some("host".to_string()),
            login: None,
            password: Some("pass".to_string()),
            login_prompt: Box::new(login_prompt_mock),
            password_prompt: Box::new(password_prompt_mock),
            host_prompt: Box::new(host_prompt_mock),
        };

        let result = init.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }

    #[test]
    fn no_provided_password_should_invoke_prompt() {
        let login_prompt_mock = make_never_called_prompt_mock();
        let password_prompt_mock = make_prompt_mock("prompt_password");
        let host_prompt_mock = make_never_called_prompt_mock();

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_initialize()
            .once()
            .returning(|| Ok(()));
        mock_workspace
            .expect_save_config_object::<BacaRelease>()
            .returning(|_| Ok(()));

        let expected_config = make_baca_config("host", "login", "prompt_password");
        let expected_cookie = expected_config.cookie.clone();
        mock_workspace
            .expect_save_config_object()
            .withf(move |x: &ConnectionConfig| *x == expected_config)
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .returning(move |_| Ok(expected_cookie.clone()));

        let init = Init {
            host: Some("host".to_string()),
            login: Some("login".to_string()),
            password: None,
            login_prompt: Box::new(login_prompt_mock),
            password_prompt: Box::new(password_prompt_mock),
            host_prompt: Box::new(host_prompt_mock),
        };

        let result = init.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }

    #[test]
    fn no_provided_login_and_password_should_invoke_prompt() {
        let login_prompt_mock = make_prompt_mock("prompt_login");
        let password_prompt_mock = make_prompt_mock("prompt_password");
        let host_prompt_mock = make_never_called_prompt_mock();

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_initialize()
            .once()
            .returning(|| Ok(()));
        mock_workspace
            .expect_save_config_object::<BacaRelease>()
            .returning(|_| Ok(()));

        let expected_config = make_baca_config("host", "prompt_login", "prompt_password");
        let expected_cookie = expected_config.cookie.clone();
        mock_workspace
            .expect_save_config_object()
            .withf(move |x: &ConnectionConfig| *x == expected_config)
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .returning(move |_| Ok(expected_cookie.clone()));

        let init = Init {
            host: Some("host".to_string()),
            login: None,
            password: None,
            login_prompt: Box::new(login_prompt_mock),
            password_prompt: Box::new(password_prompt_mock),
            host_prompt: Box::new(host_prompt_mock),
        };

        let result = init.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }

    #[test]
    fn no_provided_host_should_invoke_prompt() {
        let input_prompt_mock = make_never_called_prompt_mock();
        let password_prompt_mock = make_never_called_prompt_mock();
        let host_prompt_mock = make_prompt_mock("prompt_host");

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_initialize()
            .once()
            .returning(|| Ok(()));
        mock_workspace
            .expect_save_config_object::<BacaRelease>()
            .returning(|_| Ok(()));

        let expected_config = make_baca_config("prompt_host", "login", "pass");
        let expected_cookie = expected_config.cookie.clone();
        mock_workspace
            .expect_save_config_object()
            .withf(move |x: &ConnectionConfig| *x == expected_config)
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .returning(move |_| Ok(expected_cookie.clone()));

        let init = Init {
            host: None,
            login: Some("login".to_string()),
            password: Some("pass".to_string()),
            login_prompt: Box::new(input_prompt_mock),
            password_prompt: Box::new(password_prompt_mock),
            host_prompt: Box::new(host_prompt_mock),
        };

        let result = init.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
