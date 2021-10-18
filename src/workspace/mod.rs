pub mod config_object;
mod instance_data;
mod task_config;
pub mod workspace_dir;
pub mod workspace_paths;
mod zip;

pub use self::config_object::ConfigObject;
pub use self::instance_data::InstanceData;
pub use self::task_config::TaskConfig;
pub use self::workspace_dir::WorkspaceDir;
pub use self::workspace_paths::WorkspacePaths;
pub use self::zip::zip_file;
use crate::error::Result;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Workspace {
    fn initialize(&self) -> Result<()>;
    fn remove_workspace(&self) -> Result<()>;
    fn save_config_object<T: ConfigObject + 'static>(&self, object: &T) -> Result<()>;
    fn read_config_object<T: ConfigObject + 'static>(&self) -> Result<T>;
    fn remove_config_object<T: ConfigObject + 'static>(&self) -> Result<()>;
}
