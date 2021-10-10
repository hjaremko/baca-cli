use crate::baca::details::Language;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(test)]
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskConfig {
    pub id: String,
    pub file: PathBuf,
    pub to_zip: bool,
    pub language: Language,
    pub rename_as: Option<String>,
}

impl TaskConfig {
    #[cfg(test)]
    pub fn new(
        id: &str,
        file: &Path,
        to_zip: bool,
        language: Language,
        rename_as: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            file: file.to_owned(),
            to_zip,
            language,
            rename_as,
        }
    }
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            file: PathBuf::new(),
            to_zip: false,
            language: Language::Unsupported,
            rename_as: None,
        }
    }
}
