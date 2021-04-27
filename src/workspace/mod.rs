mod instance_data;
mod task_config;
mod zip;

use std::fs::{DirBuilder, ReadDir};
use std::io::ErrorKind;
use std::{fs, io};
use tracing::{debug, info};

pub use self::instance_data::InstanceData;
pub use self::task_config::TaskConfig;
pub use self::zip::zip_file;
use crate::baca::details::Language;
use crate::error::Error;
use crate::error::Result;

// todo: walk up dir tree until found
const BACA_DIR: &str = ".baca";
const INSTANCE_PATH: &str = ".baca/instance";
const TASK_PATH: &str = ".baca/task";

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Workspace {
    fn initialize(&self) -> Result<()>;
    fn save_instance(&self, instance: &InstanceData) -> Result<()>;
    fn read_instance(&self) -> Result<InstanceData>;
    fn read_task(&self) -> Result<TaskConfig>;
    fn save_task(
        &self,
        task_id: &str,
        filepath: &str,
        to_zip: bool,
        language: Language,
    ) -> Result<()>;
    fn remove_task(&self) -> Result<()>;
    fn remove_workspace(&self) -> Result<()>;
}

pub struct WorkspaceDir {}

impl Workspace for WorkspaceDir {
    fn initialize(&self) -> Result<()> {
        let baca_dir = check_if_initialized();

        if baca_dir.is_ok() {
            return Err(Error::WorkspaceAlreadyInitialized);
        }

        info!("Creating new {}", BACA_DIR);
        DirBuilder::new()
            .create(BACA_DIR)
            .map_err(as_config_create_error)?;

        info!("Baca directory created successfully.");
        Ok(())
    }

    fn save_instance(&self, instance: &InstanceData) -> Result<()> {
        info!("Saving instance to the workspace.");
        let serialized = serde_json::to_string(instance).expect("Instance serialization error");
        debug!("Serialized: {}", serialized);

        fs::write(INSTANCE_PATH, serialized).map_err(as_config_write_error)?;
        Ok(())
    }

    fn read_instance(&self) -> Result<InstanceData> {
        check_if_initialized()?;

        info!("Reading {}", INSTANCE_PATH);
        let serialized = fs::read_to_string(INSTANCE_PATH).map_err(as_config_read_error)?;
        debug!("Serialized: {}", serialized);

        let deserialized: InstanceData = serde_json::from_str(&serialized)?;
        debug!("Deserialized: {:?}", deserialized);
        Ok(deserialized)
    }

    fn read_task(&self) -> Result<TaskConfig> {
        check_if_initialized()?;
        info!("Reading task from workspace.");
        let serialized = fs::read_to_string(TASK_PATH).map_err(as_task_read_error)?;
        debug!("Serialized: {}", serialized);

        let deserialized: TaskConfig = serde_json::from_str(&serialized)?;
        debug!("Deserialized: {:?}", deserialized);

        info!("Read task successfully.");
        Ok(deserialized)
    }

    fn save_task(
        &self,
        task_id: &str,
        filepath: &str,
        to_zip: bool,
        language: Language,
    ) -> Result<()> {
        info!("Saving task info to {}.", TASK_PATH);

        let task = TaskConfig {
            id: task_id.to_string(),
            file: filepath.to_string(),
            language,
            to_zip,
        };
        let serialized = serde_json::to_string(&task)?;
        debug!("Serialized: {}", serialized);

        fs::write(TASK_PATH, serialized).map_err(as_task_write_error)?;
        info!("Saved task successfully.");
        Ok(())
    }

    fn remove_task(&self) -> Result<()> {
        info!("Removing task from {}.", TASK_PATH);
        fs::remove_file(TASK_PATH).map_err(as_task_remove_error)
    }

    fn remove_workspace(&self) -> Result<()> {
        info!("Removing Baca workspace.");
        fs::remove_dir_all(BACA_DIR).map_err(as_config_remove_error)
    }
}

fn check_if_initialized() -> Result<ReadDir> {
    info!("Checking if {} exists.", BACA_DIR);
    fs::read_dir(BACA_DIR).map_err(as_not_init_error)
}

fn as_not_init_error(e: io::Error) -> Error {
    match e.kind() {
        ErrorKind::NotFound => Error::WorkspaceNotInitialized,
        _ => Error::OpeningWorkspaceError(e.into()),
    }
}

fn as_config_read_error(e: io::Error) -> Error {
    match e.kind() {
        ErrorKind::NotFound => Error::WorkspaceCorrupted,
        _ => Error::OpeningWorkspaceError(e.into()),
    }
}

fn as_config_write_error(e: io::Error) -> Error {
    Error::WritingWorkspaceError(e.into())
}

fn as_config_create_error(e: io::Error) -> Error {
    Error::CreatingWorkspaceError(e.into())
}

fn as_config_remove_error(e: io::Error) -> Error {
    Error::RemovingWorkspaceError(e.into())
}

fn as_task_remove_error(e: io::Error) -> Error {
    Error::RemovingTaskError(e.into())
}

fn as_task_read_error(e: io::Error) -> Error {
    Error::ReadingTaskError(e.into())
}

fn as_task_write_error(e: io::Error) -> Error {
    Error::ReadingTaskError(e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn init_success() {
        let w = WorkspaceDir {};
        let result = w.initialize();

        assert!(result.is_ok());
        assert!(fs::read_dir(BACA_DIR).is_ok());
        assert!(fs::remove_dir_all(BACA_DIR).is_ok());
    }

    #[test]
    #[serial]
    fn init_already_initialized() {
        let w = WorkspaceDir {};
        let result = w.initialize();
        assert!(result.is_ok());

        let result = w.initialize();
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceAlreadyInitialized));
        }

        assert!(fs::read_dir(BACA_DIR).is_ok());
        assert!(fs::remove_dir_all(BACA_DIR).is_ok());
    }

    fn make_baca() -> InstanceData {
        InstanceData {
            host: "test_host".to_string(),
            login: "test_login".to_string(),
            password: "test_pass".to_string(),
            permutation: "test_perm".to_string(),
            cookie: "test_cookie".to_string(),
        }
    }

    #[test]
    #[serial]
    fn save_read_instance_success() {
        let w = WorkspaceDir {};
        let result = w.initialize();
        assert!(result.is_ok());

        let baca = make_baca();
        let result = w.save_instance(&baca);

        assert!(result.is_ok());
        assert!(fs::read(INSTANCE_PATH).is_ok());

        let result = w.read_instance();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), baca);

        assert!(fs::remove_dir_all(BACA_DIR).is_ok());
    }
}
