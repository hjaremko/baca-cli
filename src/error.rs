use crate::error::Error::{Network, Other, Protocol, WorkspaceCorrupted};
use std::{fmt, io};
use tracing::error;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    Network(Box<dyn std::error::Error>),
    CreatingWorkspace(Box<dyn std::error::Error>),
    RemovingWorkspace(Box<dyn std::error::Error>),
    RemovingConfig(Box<dyn std::error::Error>),
    ReadingConfig(Box<dyn std::error::Error>),
    SavingConfig(Box<dyn std::error::Error>),
    ReadingSource(Box<dyn std::error::Error>),
    Zipping(Box<dyn std::error::Error>),
    WorkspaceNotInitialized,
    WorkspaceCorrupted,
    WorkspaceAlreadyInitialized,
    InvalidSubmitId,
    Protocol,
    LoggedOut,
    TaskNotActive,
    InvalidTaskId(String),
    InvalidHost,
    InvalidLoginOrPassword,
    FetchingRelease,
    NoRelease,
    ApiRateLimitExceeded,
    InvalidArgument,
    UnsupportedLanguage(String),
    NoSubmitsYet,
    EditorFail(i32),
    SubmitArgumentNotProvided(String),
    InputFileDoesNotExist,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::Other(e) => format!("Error: {}", e),
            Error::Network(e) => format!("Network error: {}", e),
            Error::CreatingWorkspace(e) => format!("Error creating workspace directory: {}", e),
            Error::RemovingWorkspace(e) => format!("Error removing workspace directory: {}", e),
            Error::RemovingConfig(e) => format!("Error removing config: {}", e),
            Error::ReadingConfig(e) => format!("Error reading config: {}", e),
            Error::SavingConfig(e) => format!("Error saving config: {}", e),
            Error::ReadingSource(e) => format!("Error reading source file: {}", e),
            Error::Zipping(e) => format!("Error zipping! Error: {}", e),
            Error::WorkspaceNotInitialized => "Baca is not initialized! Type 'baca init --help' for more information.".to_owned(),
            Error::WorkspaceCorrupted => "Workspace corrupted, please delete .baca directory and initialize again.".to_owned(),
            Error::WorkspaceAlreadyInitialized => "Baca already initialized. Remove '.baca' directory if you want to change config or edit it manually.".to_owned(),
            Error::InvalidSubmitId => "Invalid submit id.".to_owned(),
            Error::Protocol => "Unfortunately, Baca still uses deprecated TLSv1 protocol which is not supported on your system. Sorry!".to_owned(),
            Error::LoggedOut => "The session cookie has expired, type 'baca refresh' to re-log and try again.".to_owned(),
            Error::TaskNotActive => "Error sending submit. Is the task still active?".to_owned(),
            Error::InvalidTaskId(id) => format!("Task no. {} does not exist.", id),
            Error::InvalidHost => "Invalid host provided. Example: for baca url 'https://baca.ii.uj.edu.pl/mn2021/', the host is 'mn2021'.".to_owned(),
            Error::InvalidLoginOrPassword => "Invalid login or password!".to_owned(),
            Error::FetchingRelease => "Error fetching releases.".to_owned(),
            Error::NoRelease => "No releases available.".to_owned(),
            Error::ApiRateLimitExceeded => "GitHub API rate limit exceeded. Try again later.".to_owned(),
            Error::InvalidArgument => "Invalid argument.".to_owned(),
            Error::UnsupportedLanguage(lang) => format!("{} is not yet supported!! Please create an issue at https://github.com/hjaremko/baca-cli/issues", lang),
            Error::NoSubmitsYet => "No submits yet!".to_owned(),
            Error::InputFileDoesNotExist => "Provided input file does not exist!".to_owned(),
            Error::EditorFail(code) => format!("Config editor failed with exit code: {}", code),
            Error::SubmitArgumentNotProvided(argument) => format!("Please provide {}. Type 'baca submit -h' for more info.", argument),
        };

        write!(f, "{}", msg)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        error!("{}", e);
        Other(e.into())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        error!("{}", e);
        WorkspaceCorrupted
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        error!("{}", e);

        if e.to_string().contains("unsupported protocol") {
            return Protocol;
        }

        Network(e.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        error!("{}", e);
        Other(e.into())
    }
}
