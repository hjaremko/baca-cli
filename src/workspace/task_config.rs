use crate::baca::details::Language;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskConfig {
    pub id: String,
    pub file: PathBuf,
    pub to_zip: bool,
    pub language: Language,
}

impl TaskConfig {
    pub fn new(id: &str, file: &Path, to_zip: bool, language: Language) -> Self {
        Self {
            id: id.to_string(),
            file: file.to_owned(),
            to_zip,
            language,
        }
    }
}
