use crate::error::Result;
#[cfg(test)]
use mockall::{automock, predicate::*};

pub use self::config_object::ConfigObject;
pub use self::connection_config::ConnectionConfig;
pub use self::no_main::remove_main;
pub use self::no_polish::make_polishless_file;
pub use self::submit_config::SubmitConfig;
pub use self::workspace_dir::WorkspaceDir;
pub use self::workspace_paths::WorkspacePaths;
pub use self::zip::zip_file;

pub mod baca_release;
pub mod config_editor;
pub mod config_object;
mod connection_config;
mod no_main;
mod no_polish;
mod submit_config;
pub mod workspace_dir;
pub mod workspace_paths;
mod zip;

#[cfg_attr(test, automock)]
pub trait Workspace {
    fn initialize(&self) -> Result<()>;
    fn check_if_initialized(&self) -> Result<()>;
    fn remove_workspace(&self) -> Result<()>;
    fn save_config_object<T: ConfigObject + 'static>(&self, object: &T) -> Result<()>;
    fn read_config_object<T: ConfigObject + 'static>(&self) -> Result<T>;
    fn remove_config_object<T: ConfigObject + 'static>(&self) -> Result<()>;
    fn get_paths(&self) -> WorkspacePaths;
}
