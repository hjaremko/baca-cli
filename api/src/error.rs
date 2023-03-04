use crate::error::Error::{Network, Other};
use std::{fmt, io};
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    Network(Box<dyn std::error::Error>),
    InvalidSubmitId,
    LoggedOut,
    TaskNotActive,
    InvalidTaskId(String),
    InvalidHost,
    InvalidLoginOrPassword,
    UnsupportedLanguage(String),
    NoSubmitsYet,
    SubmitArgumentNotProvided(String),
    NoHeader,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::Other(e) => format!("Error: {}", e),
            Error::Network(e) => format!("Network error: {}", e),
            Error::InvalidSubmitId => "Invalid submit id.".to_owned(),
            Error::LoggedOut => "The session cookie has expired, type 'baca refresh' to re-log and try again.".to_owned(),
            Error::TaskNotActive => "Error sending submit. Is the task still active?".to_owned(),
            Error::InvalidTaskId(id) => format!("Task no. {} does not exist.", id),
            Error::InvalidHost => "Invalid host provided. Example: for baca url 'https://baca.ii.uj.edu.pl/mn2021/', the host is 'mn2021'.".to_owned(),
            Error::InvalidLoginOrPassword => "Invalid login or password!".to_owned(),
            Error::UnsupportedLanguage(lang) => format!("{} is not yet supported!! Please create an issue at https://github.com/hjaremko/baca-cli/issues", lang),
            Error::NoSubmitsYet => "No submits yet!".to_owned(),
            Error::SubmitArgumentNotProvided(argument) => format!("Please provide {}. Type 'baca submit -h' for more info.", argument),
            Error::NoHeader => "No header!".to_owned(),
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

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        error!("{}", e);
        Network(e.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        error!("{}", e);
        Other(e.into())
    }
}
