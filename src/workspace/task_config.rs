use crate::baca::details::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub id: String,
    pub file: String, // todo: save absolute path
    pub to_zip: bool,
    pub language: Language,
}
