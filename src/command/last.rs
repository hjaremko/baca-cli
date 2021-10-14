use crate::baca::api::baca_api::BacaApi;
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
    fn execute<W, A>(self, workspace: &W, api: &A) -> error::Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        let instance = workspace.read_instance()?;
        let results = api.get_results(&instance)?;
        let results = Results::parse(&instance, &results);
        let last = results.submits.first().ok_or(Error::NoSubmitsYet)?;

        Details::new(&last.id).execute::<W, A>(workspace, api)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_api::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    #[test]
    fn no_submits() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(InstanceData::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_results()
            .withf(|x| *x == InstanceData::default())
            .returning(|_| Ok(r#"//OK[0,[],0,7]"#.to_string()));

        let last = Last::new();
        let result = last.execute(&mock_workspace, &mock_api);
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), Error::NoSubmitsYet));
    }
}
