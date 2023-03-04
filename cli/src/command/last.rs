// use crate::api::baca_api::BacaApi;
use crate::command::details::Details;
use crate::command::Command;
use api::error::{Error, Result};
// use crate::model::Submit;
use crate::workspace::{ConfigObject, ConnectionConfig, Workspace};
use clap::ArgMatches;
use api::api::baca_api::BacaApi;
use api::model::Submit;

pub struct Last {
    task_id: Option<String>,
}

impl Last {
    pub fn new() -> Self {
        Self { task_id: None }
    }

    pub fn with_filter(task_id: &str) -> Self {
        Self {
            task_id: Some(task_id.to_string()),
        }
    }

    fn get_last_submit<A>(&self, connection_config: &ConnectionConfig, api: &A) -> Result<Submit>
    where
        A: BacaApi,
    {
        let results = if let Some(task_id) = &self.task_id {
            api.get_results_by_task(connection_config, task_id)?
        } else {
            api.get_results(connection_config)?
        };

        Ok(results.submits.first().ok_or(Error::NoSubmitsYet)?.clone())
    }
}

impl Command for Last {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        let connection_config = ConnectionConfig::read_config(workspace)?;
        let last = self.get_last_submit(&connection_config, api)?;

        Details::new(&last.id).execute(workspace, api)
    }
}

impl From<&ArgMatches<'_>> for Last {
    fn from(args: &ArgMatches) -> Self {
        if let Some(task_id) = args.value_of("task") {
            return Last::with_filter(task_id);
        }

        Last::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::baca_api::MockBacaApi;
    use crate::model::SubmitStatus;
    use crate::model::{Results, Submit};
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
            .returning(|_| Ok(Results { submits: vec![] }));

        let last = Last::new();
        let result = last.execute(&mock_workspace, &mock_api);
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), Error::NoSubmitsYet));
    }

    #[test]
    fn one_submit() {
        let expected = Submit {
            status: SubmitStatus::InternalError,
            points: 0.0,
            lateness: None,
            accepted: 0,
            size: 123,
            timestamp: "2002".to_string(),
            language: "Java".to_string(),
            id: "3".to_string(),
            max_points: None,
            problem_name: "Test Problem".to_string(),
            link: "www.baca.pl".to_string(),
            test_results: None,
        };

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        let results = Results {
            submits: vec![expected.clone()],
        };
        mock_api
            .expect_get_results()
            .withf(|x| *x == ConnectionConfig::default())
            .returning(move |_| Ok(results.clone()));

        let submit = expected;
        mock_api
            .expect_get_submit_details()
            .withf(|x, id| *x == ConnectionConfig::default() && id == "3")
            .returning(move |_, _| Ok(submit.clone()));

        let last = Last::new();
        let result = last.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok());
    }

    #[test]
    fn three_submits() {
        let submit1 = Submit {
            status: SubmitStatus::InternalError,
            points: 0.0,
            lateness: None,
            accepted: 0,
            size: 123,
            timestamp: "2002".to_string(),
            language: "Java".to_string(),
            id: "1".to_string(),
            max_points: None,
            problem_name: "Test Problem 1".to_string(),
            link: "www.baca.pl".to_string(),
            test_results: None,
        };

        let submit2 = Submit {
            status: SubmitStatus::InternalError,
            points: 0.0,
            lateness: None,
            accepted: 0,
            size: 123,
            timestamp: "2002".to_string(),
            language: "Java".to_string(),
            id: "2".to_string(),
            max_points: None,
            problem_name: "Test Problem 2".to_string(),
            link: "www.baca.pl".to_string(),
            test_results: None,
        };

        let submit3 = Submit {
            status: SubmitStatus::InternalError,
            points: 0.0,
            lateness: None,
            accepted: 0,
            size: 123,
            timestamp: "2002".to_string(),
            language: "Java".to_string(),
            id: "3".to_string(),
            max_points: None,
            problem_name: "Test Problem 3".to_string(),
            link: "www.baca.pl".to_string(),
            test_results: None,
        };

        let all_submits = vec![submit1.clone(), submit2, submit3];

        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        let results = Results {
            submits: all_submits,
        };
        mock_api
            .expect_get_results()
            .withf(|x| *x == ConnectionConfig::default())
            .returning(move |_| Ok(results.clone()));

        let submit = submit1;
        mock_api
            .expect_get_submit_details()
            .withf(|x, id| *x == ConnectionConfig::default() && id == "1")
            .returning(move |_, _| Ok(submit.clone()));

        let last = Last::new();
        let result = last.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok());
    }
}
