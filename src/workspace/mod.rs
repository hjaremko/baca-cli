mod instance_data;
mod task_config;
mod zip;

use serde::Serialize;
use std::fs::DirBuilder;
use std::io::ErrorKind;
use std::{fs, io};
use tracing::{debug, info};

pub use self::instance_data::InstanceData;
pub use self::task_config::TaskConfig;
pub use self::zip::zip_file;

use crate::error::Error;
use crate::error::Result;

// todo: walk up dir tree until found
#[derive(Clone)]
struct WorkspacePaths {
    root_path: PathBuf,
}

impl WorkspacePaths {
    pub fn new() -> Self {
        Self {
            root_path: Path::new(".").to_path_buf(),
        }
    }

    fn _with_root(root_path: &Path) -> Self {
        Self {
            root_path: root_path.to_path_buf(),
        }
    }

    pub fn baca_dir(&self) -> PathBuf {
        self.root_path.join(".baca")
    }

    pub fn instance_path(&self) -> PathBuf {
        self.baca_dir().join("instance")
    }

    pub fn task_path(&self) -> PathBuf {
        self.baca_dir().join("task")
    }
}

#[cfg(test)]
use mockall::{automock, predicate::*};
use std::path::{Path, PathBuf};

#[cfg_attr(test, automock)]
pub trait Workspace {
    fn initialize(&self) -> Result<()>;
    fn save_instance(&self, instance: &InstanceData) -> Result<()>;
    fn read_instance(&self) -> Result<InstanceData>;
    fn read_task(&self) -> Result<TaskConfig>;
    fn save_task(&self, task_config: &TaskConfig) -> Result<()>;
    fn remove_task(&self) -> Result<()>;
    fn remove_workspace(&self) -> Result<()>;
    fn save_object<T: 'static + Serialize>(&self, filename: &str, content: &T) -> Result<()>;
    fn read_file(&self, filename: &str) -> Result<String>;
}

pub struct WorkspaceDir {
    paths: WorkspacePaths,
}

impl WorkspaceDir {
    pub fn new() -> Self {
        Self {
            paths: WorkspacePaths::new(),
        }
    }

    fn _with_paths(paths: WorkspacePaths) -> Self {
        Self { paths }
    }

    fn check_if_already_initialized(&self) -> Result<()> {
        let path = self.paths.baca_dir();
        info!("Checking if {} exists.", path.to_str().unwrap());

        if path.is_dir() {
            return Err(Error::WorkspaceAlreadyInitialized);
        }

        Ok(())
    }

    fn check_if_initialized(&self) -> Result<()> {
        let path = self.paths.baca_dir();
        info!("Checking if {} exists.", path.to_str().unwrap());

        if !path.exists() {
            return Err(Error::WorkspaceNotInitialized);
        }

        Ok(())
    }
}

impl Workspace for WorkspaceDir {
    fn initialize(&self) -> Result<()> {
        self.check_if_already_initialized()?;

        let path = self.paths.baca_dir();
        info!("Creating new {}", path.to_str().unwrap());

        DirBuilder::new()
            .create(path)
            .map_err(as_config_create_error)?;

        info!("Baca directory created successfully.");
        Ok(())
    }

    fn save_instance(&self, instance: &InstanceData) -> Result<()> {
        self.check_if_initialized()?;

        info!("Saving instance to the workspace.");
        let serialized = serde_json::to_string(instance).expect("Instance serialization error");
        debug!("Serialized: {}", serialized);

        fs::write(self.paths.instance_path(), serialized).map_err(as_config_write_error)?;
        Ok(())
    }

    fn read_instance(&self) -> Result<InstanceData> {
        self.check_if_initialized()?;

        info!("Reading {}", self.paths.instance_path().to_str().unwrap());
        let serialized =
            fs::read_to_string(self.paths.instance_path()).map_err(as_config_read_error)?;
        debug!("Serialized: {}", serialized);

        let deserialized: InstanceData = serde_json::from_str(&serialized)?;
        debug!("Deserialized: {:?}", deserialized);
        Ok(deserialized)
    }

    fn read_task(&self) -> Result<TaskConfig> {
        self.check_if_initialized()?;
        info!("Reading task from workspace.");
        let serialized = fs::read_to_string(self.paths.task_path()).map_err(as_task_read_error)?;
        debug!("Serialized: {}", serialized);

        let deserialized: TaskConfig = serde_json::from_str(&serialized)?;
        debug!("Deserialized: {:?}", deserialized);

        info!("Read task successfully.");
        Ok(deserialized)
    }

    // todo: prompt for override
    // todo: refactor to_zip
    // todo: get struct as argument
    fn save_task(&self, task_config: &TaskConfig) -> Result<()> {
        self.check_if_initialized()?;
        // todo: check correctness of other fields (TaskValidator?)
        // let input_file_path = Path::new(filepath);
        //
        // if !input_file_path.exists() {
        //     return Err(Error::InputFileDoesNotExist);
        // }

        info!(
            "Saving task info to {}.",
            self.paths.task_path().to_str().unwrap()
        );

        let serialized = serde_json::to_string(&task_config)?;
        debug!("Serialized: {}", serialized);

        fs::write(self.paths.task_path(), serialized).map_err(as_task_write_error)?;
        println!("Task config has been saved.");
        Ok(())
    }

    fn remove_task(&self) -> Result<()> {
        info!(
            "Removing task from {}.",
            self.paths.task_path().to_str().unwrap()
        );
        fs::remove_file(self.paths.task_path()).map_err(as_task_remove_error)
    }

    fn remove_workspace(&self) -> Result<()> {
        info!("Removing Baca workspace.");
        fs::remove_dir_all(self.paths.baca_dir()).map_err(as_config_remove_error)
    }

    fn save_object<T: 'static + Serialize>(&self, filename: &str, content: &T) -> Result<()> {
        info!(
            "Saving object as {} to {}.",
            filename,
            self.paths.baca_dir().to_str().unwrap()
        );
        let serialized = serde_json::to_string(&content)?;
        debug!("Serialized: {}", serialized);
        let path = self.paths.baca_dir().join(filename);
        fs::write(path, serialized).map_err(as_task_write_error)?;
        Ok(())
    }

    fn read_file(&self, filename: &str) -> Result<String> {
        info!(
            "Reading file {} from {}.",
            filename,
            self.paths.baca_dir().to_str().unwrap()
        );
        let path = self.paths.baca_dir().join(filename);
        let serialized = fs::read_to_string(path).map_err(as_task_read_error)?;
        Ok(serialized)
    }
}

fn as_config_read_error(e: io::Error) -> Error {
    match e.kind() {
        ErrorKind::NotFound => Error::WorkspaceCorrupted,
        _ => Error::OpeningWorkspace(e.into()),
    }
}

fn as_config_write_error(e: io::Error) -> Error {
    Error::WritingWorkspace(e.into())
}

fn as_config_create_error(e: io::Error) -> Error {
    Error::CreatingWorkspace(e.into())
}

fn as_config_remove_error(e: io::Error) -> Error {
    Error::RemovingWorkspace(e.into())
}

fn as_task_remove_error(e: io::Error) -> Error {
    Error::RemovingTask(e.into())
}

fn as_task_read_error(e: io::Error) -> Error {
    Error::ReadingTask(e.into())
}

fn as_task_write_error(e: io::Error) -> Error {
    Error::ReadingTask(e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::details::Language;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use predicates::prelude::*;

    fn make_temp_workspace(
    ) -> std::result::Result<(TempDir, WorkspacePaths, WorkspaceDir), Box<dyn std::error::Error>>
    {
        let temp_dir = assert_fs::TempDir::new()?;
        let mock_paths = WorkspacePaths::_with_root(temp_dir.path());
        let workspace = WorkspaceDir::_with_paths(mock_paths.clone());
        Ok((temp_dir, mock_paths, workspace))
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
    fn init_success() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        workspace.initialize().unwrap();

        assert!(fs::read_dir(mock_paths.baca_dir()).is_ok());
        temp_dir.close().unwrap();
    }

    #[test]
    fn init_already_initialized() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        let result = workspace.initialize();
        assert!(result.is_ok());

        let result = workspace.initialize();
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceAlreadyInitialized));
        }

        assert!(fs::read_dir(mock_paths.baca_dir()).is_ok());
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_read_instance_success() {
        let (temp_dir, _, workspace) = make_temp_workspace().unwrap();
        let baca = make_baca();

        workspace.initialize().unwrap();
        workspace.save_instance(&baca).unwrap();

        let result = workspace.read_instance().unwrap();
        assert_eq!(result, baca);
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_instance_not_initialized() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        let baca = make_baca();
        let result = workspace.save_instance(&baca);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceNotInitialized));
        }
        assert!(predicate::path::missing().eval(mock_paths.instance_path().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_instance_should_override() {
        let (temp_dir, _, workspace) = make_temp_workspace().unwrap();
        let baca_first = make_baca();
        let mut baca_second = make_baca();
        baca_second.host = "other_host".to_string();

        workspace.initialize().unwrap();
        workspace.save_instance(&baca_first).unwrap();
        assert_eq!(workspace.read_instance().unwrap(), baca_first);
        workspace.save_instance(&baca_second).unwrap();
        assert_eq!(workspace.read_instance().unwrap(), baca_second);

        temp_dir.close().unwrap();
    }

    #[test]
    fn save_read_task_success() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let input_file = temp_dir.child("foo.sh");
        input_file.touch().unwrap();
        let expected_task_config =
            TaskConfig::new("2", input_file.path(), false, Language::Bash, None);

        workspace.initialize().unwrap();
        workspace.save_task(&expected_task_config).unwrap();

        assert_eq!(workspace.read_task().unwrap(), expected_task_config);
        assert!(predicate::path::exists().eval(mock_paths.task_path().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn read_corrupted_task() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let corrupted_task_config = ChildPath::new(mock_paths.task_path());

        workspace.initialize().unwrap();
        corrupted_task_config.write_str("invalid config").unwrap();
        let result = workspace.read_task();

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceCorrupted));
        }
        assert!(predicate::path::exists().eval(mock_paths.task_path().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn read_no_task_exists() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        workspace.initialize().unwrap();
        let result = workspace.read_task();

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::ReadingTask(_)));
        }
        assert!(predicate::path::missing().eval(mock_paths.task_path().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_task_not_initialized() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        let result = workspace.save_task(&TaskConfig::new(
            "2",
            Path::new("foo.txt"),
            true,
            Language::Bash,
            None,
        ));

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceNotInitialized));
        }
        assert!(predicate::path::missing().eval(mock_paths.task_path().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_task_should_override() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let input_file = temp_dir.child("foo.sh");
        input_file.touch().unwrap();
        let task_config_first =
            TaskConfig::new("2", input_file.path(), false, Language::Bash, None);
        let task_config_second =
            TaskConfig::new("3", Path::new("bar.cpp"), false, Language::Cpp, None);

        workspace.initialize().unwrap();
        workspace.save_task(&task_config_first).unwrap();

        workspace.save_task(&task_config_second).unwrap();

        assert_eq!(workspace.read_task().unwrap(), task_config_second);
        assert!(predicate::path::exists().eval(mock_paths.task_path().as_path()));
        temp_dir.close().unwrap();
    }
    // todo: tests for removing and saving objects
}
