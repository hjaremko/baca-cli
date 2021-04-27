use crate::baca::api::baca_service::BacaApi;
use crate::command::Command;
use crate::error::Result;
use crate::model;
use crate::workspace::Workspace;
use tracing::info;

pub struct Tasks {}

impl Tasks {
    pub fn new() -> Self {
        Tasks {}
    }
}

impl Command for Tasks {
    fn execute<W: Workspace, A: BacaApi>(self) -> Result<()> {
        info!("Getting all tasks.");
        let instance = W::read_instance()?;
        let tasks = A::get_tasks(&instance)?;
        let tasks = model::Tasks::parse(&tasks);

        tasks.print();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_service::MockBacaApi;
    use crate::workspace::{InstanceData, MockWorkspace};

    // todo: tests::utils
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
    fn success_test() {
        let ctx_read = MockWorkspace::read_instance_context();
        ctx_read.expect().returning(|| Ok(make_mock_instance()));

        let ctx_api = MockBacaApi::get_tasks_context();
        ctx_api
            .expect()
            .once()
            .withf(|x| *x == make_mock_instance())
            .returning(|_| Ok(r#"//OK[0,12,11,10,3,3,9,8,7,3,3,6,5,4,3,3,2,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","1","Metoda parametryzacji","12","2","Metoda parametryzacji torusów","4","id","nazwa","liczba OK"],0,7]"#.to_string()));

        let tasks = Tasks::new();
        let result = tasks.execute::<MockWorkspace, MockBacaApi>();
        assert!(result.is_ok())
    }
}
