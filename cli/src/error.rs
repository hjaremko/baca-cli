pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
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
    FetchingRelease,
    NoRelease,
    ApiRateLimitExceeded,
    InvalidArgument,
    EditorFail(i32),
    InputFileDoesNotExist,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::Other(e) => format!("Error: {}", e),
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
            Error::FetchingRelease => "Error fetching releases.".to_owned(),
            Error::NoRelease => "No releases available.".to_owned(),
            Error::ApiRateLimitExceeded => "GitHub API rate limit exceeded. Try again later.".to_owned(),
            Error::InvalidArgument => "Invalid argument.".to_owned(),
            Error::InputFileDoesNotExist => "Provided input file does not exist!".to_owned(),
            Error::EditorFail(code) => format!("Config editor failed with exit code: {}", code),
        };

        write!(f, "{}", msg)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        error!("{}", e);
        WorkspaceCorrupted
    }
}
