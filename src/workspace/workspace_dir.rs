use crate::error::Error;
use crate::error::Result;
use crate::workspace::{ConfigObject, Workspace, WorkspacePaths};
use std::fs::DirBuilder;
use std::io::ErrorKind;
use std::{fs, io};
use tracing::{debug, info};

pub struct WorkspaceDir {
    paths: WorkspacePaths,
}

impl WorkspaceDir {
    pub fn new() -> Self {
        Self {
            paths: WorkspacePaths::new(),
        }
    }

    pub(crate) fn _with_paths(paths: WorkspacePaths) -> Self {
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

    fn remove_workspace(&self) -> Result<()> {
        self.check_if_initialized()?;
        info!("Removing Baca workspace.");
        fs::remove_dir_all(self.paths.baca_dir()).map_err(as_config_remove_error)
    }

    fn save_config_object<T>(&self, object: &T) -> Result<()>
    where
        T: ConfigObject + 'static,
    {
        self.check_if_initialized()?;
        let path = self.paths.config_path::<T>();

        info!("Saving object {}", path.to_str().unwrap());

        let serialized = serde_yaml::to_string(object)?;
        debug!("Serialized: {}", serialized);
        fs::write(path, serialized).map_err(|e| Error::Other(e.into()))
    }

    fn read_config_object<T>(&self) -> Result<T>
    where
        T: ConfigObject + 'static,
    {
        self.check_if_initialized()?;
        let path = self.paths.config_path::<T>();

        info!("Reading {}", path.to_str().unwrap());
        let serialized = fs::read_to_string(path).map_err(as_config_read_error)?;
        debug!("Serialized: {}", serialized);

        let deserialized = serde_yaml::from_str::<T>(&serialized)?;
        debug!("Deserialized: {:?}", deserialized);

        Ok(deserialized)
    }

    fn remove_config_object<T>(&self) -> Result<()>
    where
        T: ConfigObject + 'static,
    {
        let path = self.paths.config_path::<T>();

        info!("Removing config file {}", path.to_str().unwrap());
        fs::remove_file(path).map_err(|e| Error::Other(e.into()))
    }
}

fn as_config_read_error(e: io::Error) -> Error {
    match e.kind() {
        ErrorKind::NotFound => Error::WorkspaceCorrupted,
        _ => Error::OpeningWorkspace(e.into()),
    }
}

fn as_config_create_error(e: io::Error) -> Error {
    Error::CreatingWorkspace(e.into())
}

fn as_config_remove_error(e: io::Error) -> Error {
    Error::RemovingWorkspace(e.into())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::workspace::InstanceData;
    use assert_fs::TempDir;

    pub fn make_temp_workspace(
    ) -> std::result::Result<(TempDir, WorkspacePaths, WorkspaceDir), Box<dyn std::error::Error>>
    {
        let temp_dir = assert_fs::TempDir::new()?;
        let mock_paths = WorkspacePaths::_with_root(temp_dir.path());
        let workspace = WorkspaceDir::_with_paths(mock_paths.clone());
        Ok((temp_dir, mock_paths, workspace))
    }

    pub fn make_baca() -> InstanceData {
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
    // todo: tests for removing and saving objects
}
