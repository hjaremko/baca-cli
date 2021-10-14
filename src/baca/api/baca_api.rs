use crate::error::Result;
use crate::model::Task;
use crate::workspace::InstanceData;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait BacaApi {
    fn get_cookie(&self, instance: &InstanceData) -> Result<String>;
    fn get_submit_details(&self, instance: &InstanceData, submit_id: &str) -> Result<String>;
    fn get_results(&self, instance: &InstanceData) -> Result<String>;
    fn get_tasks(&self, instance: &InstanceData) -> Result<String>;
    fn submit(&self, instance: &InstanceData, task: &Task, file_path: &str) -> Result<()>;
}
