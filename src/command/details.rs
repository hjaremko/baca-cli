use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error::Result;
use crate::workspace::Workspace;

use clap::ArgMatches;
use tracing::info;

pub struct Details {
    submit_id: String,
}

impl Details {
    pub fn new(submit_id: &str) -> Self {
        Details {
            submit_id: submit_id.to_string(),
        }
    }
}

impl From<&ArgMatches<'_>> for Details {
    fn from(args: &ArgMatches) -> Self {
        let submit_id = args.value_of("id").unwrap();
        Self {
            submit_id: submit_id.to_string(),
        }
    }
}

impl Command for Details {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        info!("Printing details for submit: {}", self.submit_id);

        let instance = workspace.read_instance()?;
        let submit = api.get_submit_details(&instance, &self.submit_id)?;

        submit.print_with_tests();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::baca_api::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    #[test]
    fn no_tasks_yet_should_return_error() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(InstanceData::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_submit_details()
            .once()
            .withf(|x, id| *x == InstanceData::default() && id == "2888")
            .returning(|_, _| Err(crate::error::Error::InvalidSubmitId));

        let details = Details {
            submit_id: "2888".to_string(),
        };
        let result = details.execute(&mock_workspace, &mock_api);
        assert!(result.is_err(), "result = {:?}", result);
    }
}
