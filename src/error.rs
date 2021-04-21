use crate::error::Error::{WorkspaceCorrupted, NetworkError};
use std::fmt;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    // Other(Box<dyn std::error::Error>),
    NetworkError(Box<dyn std::error::Error>),
    CreatingWorkspaceError(Box<dyn std::error::Error>),
    OpeningWorkspaceError(Box<dyn std::error::Error>),
    WritingWorkspaceError(Box<dyn std::error::Error>),
    RemovingTaskError(Box<dyn std::error::Error>),
    ReadingTaskError(Box<dyn std::error::Error>),
    WorkspaceNotInitialized,
    WorkspaceCorrupted,
    WorkspaceAlreadyInitialized,
    InvalidSubmitId,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            // Error::Other(e) => write!(f, "Error: {}", e),
            Error::NetworkError(e) => format!("Network error: {}", e),
            Error::CreatingWorkspaceError(e) => format!("Error creating workspace directory: {}", e),
            Error::OpeningWorkspaceError(e) => format!("Error opening workspace directory: {}", e),
            Error::WritingWorkspaceError(e) => format!("Error writing config to the workspace directory: {}", e),
            Error::RemovingTaskError(e) => format!("Error removing task config: {}", e),
            Error::ReadingTaskError(e) => format!("Error reading task config: {}", e),
            Error::WorkspaceNotInitialized => "Baca is not initialized! Type 'baca init --help' for more information.".to_owned(),
            Error::WorkspaceCorrupted => "Workspace corrupted, please delete .baca directory and initialize again.".to_owned(),
            Error::WorkspaceAlreadyInitialized => "Baca already initialized. Remove '.baca' directory if you want to change config or edit it manually.".to_owned(),
            Error::InvalidSubmitId => "Invalid submit id.".to_owned(),
        };

        write!(f, "{}", msg)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        tracing::error!("{}", e);
        WorkspaceCorrupted
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        tracing::error!("{}", e);
        NetworkError(e.into())
    }
}

