use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::model::Results;
use crate::workspace::{ConfigObject, ConnectionConfig, Workspace};
use tracing::info;

pub struct Log {
    pub last_n: String,
    pub task_id: Option<String>,
}

impl Log {
    pub fn new(last_n: &str, task_id: &Option<u32>) -> Self {
        Log {
            last_n: last_n.to_string(),
            task_id: task_id.map(|x| x.to_string()),
        }
    }

    fn fetch_logs<A>(&self, api: &A, connection_config: &ConnectionConfig) -> Result<Results>
    where
        A: BacaApi,
    {
        Ok(if let Some(task_id) = &self.task_id {
            api.get_results_by_task(connection_config, task_id)?
        } else {
            api.get_results(connection_config)?
        })
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
        let connection_config = ConnectionConfig::read_config(workspace)?;
        let results = self.fetch_logs(api, &connection_config)?;

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
    use crate::workspace::{ConnectionConfig, MockWorkspace};

    #[test]
    fn no_submits() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_results()
            .withf(|x| *x == ConnectionConfig::default())
            .returning(|_| Ok(Results::default()));

        let log = Log::new("10", &None);
        let result = log.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
