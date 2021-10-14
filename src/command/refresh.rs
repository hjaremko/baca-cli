use crate::baca::api::baca_api::BacaApi;
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
    fn execute<W, A>(self, workspace: &W, api: &A) -> error::Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        info!("Refreshing Baca session.");
        let mut instance = workspace.read_instance()?;
        instance.cookie = api.get_cookie(&instance)?;
        workspace.save_instance(&instance)?;

        println!("New session obtained.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_api::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    #[test]
    fn refresh_success_test() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(InstanceData::default()));
        mock_workspace
            .expect_save_instance()
            .once()
            .withf(|x| {
                let mut expected = InstanceData::default();
                expected.cookie = "ok_cookie".to_string();

                *x == expected
            })
            .returning(|_| Ok(()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_cookie()
            .withf(|x| *x == InstanceData::default())
            .returning(|_| Ok("ok_cookie".to_string()));

        let refresh = Refresh::new();
        let result = refresh.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
