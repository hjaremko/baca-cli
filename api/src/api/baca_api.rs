use crate::error::Result;
use crate::model::{Results, Submit, Task, Tasks};
use crate::network::ConnectionConfig;
use crate::model::Language;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait BacaApi {
    fn get_cookie(&self, connection_config: &ConnectionConfig) -> Result<String>;
    fn get_submit_details(
        &self,
        connection_config: &ConnectionConfig,
        submit_id: &str,
    ) -> Result<Submit>;
    fn get_results(&self, connection_config: &ConnectionConfig) -> Result<Results>;
    fn get_results_by_task(
        &self,
        connection_config: &ConnectionConfig,
        task_id: &str,
    ) -> Result<Results>;
    fn get_tasks(&self, connection_config: &ConnectionConfig) -> Result<Tasks>;
    fn submit(
        &self,
        connection_config: &ConnectionConfig,
        task: &Task,
        file_path: &str,
    ) -> Result<()>;
    fn get_allowed_language(
        &self,
        connection_config: &ConnectionConfig,
        task_id: &str,
    ) -> Result<Option<Language>>;
}
