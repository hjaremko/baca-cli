use crate::baca::api::baca_service::BacaApi;
use crate::command::Command;
use crate::workspace::Workspace;
use crate::{baca, error, workspace};
use clap::ArgMatches;
use tracing::{debug, info};

pub struct Init {
    host: String,
    login: String,
    password: String,
}

impl From<&ArgMatches<'_>> for Init {
    fn from(args: &ArgMatches) -> Self {
        let host = args.value_of("host").unwrap();
        let login = args.value_of("login").unwrap();
        let password = args.value_of("password").unwrap();

        Self {
            host: host.to_string(),
            login: login.to_string(),
            password: password.to_string(),
        }
    }
}

impl Command for Init {
    // todo: -r to override current config
    fn execute<W: Workspace, A: BacaApi>(self) -> error::Result<()> {
        info!("Initializing Baca workspace.");
        debug!("Host: {}", self.host);
        debug!("Login: {}", self.login);
        debug!("Password: {}", self.password);

        let mut instance = workspace::InstanceData {
            host: self.host,
            login: self.login,
            password: self.password,
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
            login: "login".to_string(),
            password: "pass".to_string(),
        };
        let result = init.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }
}

//todo: fail cases
