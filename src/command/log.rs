use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::model::Results;
use crate::workspace::{InstanceData, Workspace};
use clap::ArgMatches;
use tracing::info;

pub struct Log {
    last_n: String,
    task_id: Option<String>,
}

impl Log {
    pub fn new(last_n: &str) -> Self {
        Log {
            last_n: last_n.to_string(),
            task_id: None,
        }
    }

    pub fn add_filter(mut self, task_id: &str) -> Self {
        self.task_id = Some(task_id.to_string());
        self
    }

    fn fetch_logs<A>(&self, api: &A, instance: &InstanceData) -> Result<Results>
    where
        A: BacaApi,
    {
        Ok(if let Some(task_id) = &self.task_id {
            api.get_results_by_task(instance, task_id)?
        } else {
            api.get_results(instance)?
        })
    }
}

impl From<&ArgMatches<'_>> for Log {
    fn from(args: &ArgMatches) -> Self {
        let last_n = args.value_of("amount").unwrap();
        let log = Self::new(last_n);

        if let Some(task_id) = args.value_of("task") {
            return log.add_filter(task_id);
        }

        log
    }
}

impl Command for Log {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        let n = to_int(&self.last_n)?;
        info!("Fetching {} logs.", n);
        let instance = workspace.read_instance()?;
        let results = self.fetch_logs(api, &instance)?;

        results.print(n);
        Ok(())
    }
}

fn to_int(n: &str) -> Result<usize> {
    n.parse().map_err(|_| Error::InvalidArgument)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::baca_api::MockBacaApi;
    use crate::model::Results;
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
            .returning(|_| Ok(Results::default()));

        let log = Log::new("10");
        let result = log.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
