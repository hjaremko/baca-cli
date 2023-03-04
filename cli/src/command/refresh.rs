// use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error;
use crate::workspace::{ConfigObject, ConnectionConfig, Workspace};
use tracing::info;
use api::api::baca_api;
use api::api::baca_api::BacaApi;

pub struct Refresh {}

impl Refresh {
    pub fn new() -> Self {
        Refresh {}
    }
}

impl Command for Refresh {
    fn execute<W, A>(self, workspace: &W, api: &A) -> api::error::Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        info!("Refreshing Baca session.");
        let mut connection_config = ConnectionConfig::read_config(workspace)?;
        connection_config.cookie = api.get_cookie(&connection_config)?;
        connection_config.save_config(workspace)?;

        println!("New session obtained.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::baca_api::MockBacaApi;
    use crate::workspace::{ConnectionConfig, MockWorkspace};

    #[test]
    fn refresh_success_test() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));
        mock_workspace
            .expect_save_config_object()
            .once()
            .withf(|x: &ConnectionConfig| {
                let mut expected = ConnectionConfig::default();
                expected.cookie = "ok_cookie".to_string();

                *x == expected
            })
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .withf(|x| *x == ConnectionConfig::default())
            .returning(|_| Ok("ok_cookie".to_string()));

        let refresh = Refresh::new();
        let result = refresh.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
