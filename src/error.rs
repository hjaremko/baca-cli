use crate::error::Error::{NetworkError, ProtocolError, WorkspaceCorrupted};
use std::fmt;
use tracing::error;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    // Other(Box<dyn std::error::Error>),
    NetworkError(Box<dyn std::error::Error>),
    CreatingWorkspaceError(Box<dyn std::error::Error>),
    OpeningWorkspaceError(Box<dyn std::error::Error>),
    WritingWorkspaceError(Box<dyn std::error::Error>),
    RemovingWorkspaceError(Box<dyn std::error::Error>),
    RemovingTaskError(Box<dyn std::error::Error>),
    ReadingTaskError(Box<dyn std::error::Error>),
    ReadingSourceError(Box<dyn std::error::Error>),
    WorkspaceNotInitialized,
    WorkspaceCorrupted,
    WorkspaceAlreadyInitialized,
    InvalidSubmitId,
    ProtocolError,
    LoggedOutError,
    SubmitError,
    InvalidTaskId(String),
    InvalidHost,
    InvalidLoginOrPassword,
    _FetchingReleaseError,
    _NoRelease,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            // Error::Other(e) => write!(f, "Error: {}", e),
            Error::NetworkError(e) => format!("Network error: {}", e),
            Error::CreatingWorkspaceError(e) => format!("Error creating workspace directory: {}", e),
            Error::OpeningWorkspaceError(e) => format!("Error opening workspace directory: {}", e),
            Error::WritingWorkspaceError(e) => format!("Error writing config to the workspace directory: {}", e),
            Error::RemovingWorkspaceError(e) => format!("Error removing workspace directory: {}", e),
            Error::RemovingTaskError(e) => format!("Error removing task config: {}", e),
            Error::ReadingTaskError(e) => format!("Error reading task config: {}", e),
            Error::ReadingSourceError(e) => format!("Error reading source file: {}", e),
            Error::WorkspaceNotInitialized => "Baca is not initialized! Type 'baca init --help' for more information.".to_owned(),
            Error::WorkspaceCorrupted => "Workspace corrupted, please delete .baca directory and initialize again.".to_owned(),
            Error::WorkspaceAlreadyInitialized => "Baca already initialized. Remove '.baca' directory if you want to change config or edit it manually.".to_owned(),
            Error::InvalidSubmitId => "Invalid submit id.".to_owned(),
            Error::ProtocolError => "Unfortunately, Baca still uses deprecated TSLv1 protocol which is not supported on your system. Sorry!".to_owned(),
            Error::LoggedOutError => "The session cookie has expired, type 'baca refresh' to re-log and try again.".to_owned(),
            Error::SubmitError => "Error sending submit. Is the task still active?".to_owned(),
            Error::InvalidTaskId(id) => format!("Task no. {} does not exist.", id),
            Error::InvalidHost => "Invalid host provided. Example: for baca url 'https://baca.ii.uj.edu.pl/mn2021/', the host is 'mn2021'.".to_owned(),
            Error::InvalidLoginOrPassword => "Invalid login or password!".to_owned(),
            Error::_FetchingReleaseError => "Error fetching releases.".to_owned(),
            Error::_NoRelease => "No releases available.".to_owned(),
        };

        write!(f, "{}", msg)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        error!("{}", e);
        WorkspaceCorrupted
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        error!("{}", e);

        if e.to_string().contains("unsupported protocol") {
            return ProtocolError;
        }

        NetworkError(e.into())
    }
}
