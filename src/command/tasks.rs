use crate::api::baca_api::BacaApi;
use crate::command::Command;
use crate::error::Result;
use crate::workspace::Workspace;
use tracing::info;

pub struct Tasks {}

impl Tasks {
    pub fn new() -> Self {
        Tasks {}
    }
}

impl Command for Tasks {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        info!("Getting all tasks.");
        let instance = workspace.read_instance()?;
        let tasks = api.get_tasks(&instance)?;

        tasks.print();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::baca_api::MockBacaApi;
    use crate::model;
    use crate::model::{Language, Task};
    use crate::workspace::{InstanceData, MockWorkspace};

    #[test]
    fn success_test() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(InstanceData::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_tasks()
            .once()
            .withf(|x| *x == InstanceData::default())
            .returning(|_| {
                Ok(model::Tasks {
                    tasks: vec![
                        Task {
                            id: "1".to_string(),
                            language: Language::Unsupported,
                            problem_name: "Test 1".to_string(),
                            overall_oks: 5,
                        },
                        Task {
                            id: "2".to_string(),
                            language: Language::CppWithFileSupport,
                            problem_name: "Test 2".to_string(),
                            overall_oks: 4,
                        },
                        Task {
                            id: "3".to_string(),
                            language: Language::Cpp,
                            problem_name: "Test 3".to_string(),
                            overall_oks: 3,
                        },
                        Task {
                            id: "4".to_string(),
                            language: Language::Ada,
                            problem_name: "Test 4".to_string(),
                            overall_oks: 2,
                        },
                        Task {
                            id: "5".to_string(),
                            language: Language::Bash,
                            problem_name: "Test 5".to_string(),
                            overall_oks: 1,
                        },
                    ],
                })
            });

        let tasks = Tasks::new();
        let result = tasks.execute(&mock_workspace, &mock_api);
        assert!(result.is_ok())
    }
}
