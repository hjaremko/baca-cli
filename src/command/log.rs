use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::workspace::Workspace;
use clap::ArgMatches;
use tracing::info;

pub struct Log {
    last_n: String,
}

impl Log {
    pub fn new(last_n: &str) -> Self {
        Log {
            last_n: last_n.to_string(),
        }
    }
}

impl From<&ArgMatches<'_>> for Log {
    fn from(args: &ArgMatches) -> Self {
        let last_n = args.value_of("amount").unwrap().to_string();
        Self { last_n }
    }
}

// todo: if task is configured, filter logs, add --all switch
impl Command for Log {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        let n = to_int(self.last_n)?;
        info!("Fetching {} logs.", n);
        let instance = workspace.read_instance()?;
        let results = api.get_results(&instance)?;

        results.print(n);
        Ok(())
    }
}

fn to_int(n: String) -> Result<usize> {
    let n = n.parse::<usize>();
    match n {
        Ok(n) => Ok(n),
        Err(_) => Err(Error::InvalidArgument),
    }
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
            .returning(|_| Ok(Results { submits: vec![] }));

        let log = Log::new("10");
        let result = log.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
