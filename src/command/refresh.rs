use crate::baca::api::baca_service::BacaApi;
use crate::command::Command;
use crate::error;
use crate::workspace::Workspace;
use tracing::info;

pub struct Refresh {}

impl Refresh {
    pub fn new() -> Self {
        Refresh {}
    }
}

impl Command for Refresh {
    fn execute<W: Workspace, A: BacaApi>(self) -> error::Result<()> {
        info!("Refreshing Baca session.");
        let mut instance = W::read_instance()?;
        instance.cookie = A::get_cookie(&instance)?;
        W::save_instance(&instance)?;

        println!("New session obtained.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_service::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    fn make_mock_instance() -> InstanceData {
        InstanceData {
            host: "host".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: "perm".to_string(),
            cookie: "invalid".to_string(),
        }
    }

    #[test]
    #[serial]
    fn refresh_success_test() {
        let ctx_read = MockWorkspace::read_instance_context();
        ctx_read.expect().returning(|| Ok(make_mock_instance()));

        let ctx_save = MockWorkspace::save_instance_context();
        ctx_save
            .expect()
            .once()
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

        let refresh = Refresh::new();
        let result = refresh.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }
}
