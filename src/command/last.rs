use crate::baca::api::baca_service::BacaApi;
use crate::command::details::Details;
use crate::command::Command;
use crate::error;
use crate::error::Error;
use crate::model::Results;
use crate::workspace::Workspace;

pub struct Last {}

impl Last {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Last {
    fn execute<W: Workspace, A: BacaApi>(self, workspace: &W) -> error::Result<()> {
        let instance = workspace.read_instance()?;
        let results = A::get_results(&instance)?;
        let results = Results::parse(&instance, &results);
        let last = results.submits.first().ok_or(Error::NoSubmitsYet)?;

        Details::new(&last.id).execute::<W, A>(workspace)
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
    fn no_submits() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(make_mock_instance()));

        let ctx_api = MockBacaApi::get_results_context();
        ctx_api
            .expect()
            .withf(|x| *x == make_mock_instance())
            .returning(|_| Ok(r#"//OK[0,[],0,7]"#.to_string()));

        let last = Last::new();
        let result = last.execute::<MockWorkspace, MockBacaApi>(&mock_workspace);
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), Error::NoSubmitsYet));
    }
}
