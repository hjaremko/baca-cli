use crate::model::Language;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub language: Language,
    pub problem_name: String,
    pub overall_oks: i32,
}

impl Task {
    #[cfg(test)]
    pub fn new(id: &str, language: Language, problem_name: &str, overall_oks: i32) -> Self {
        Self {
            id: id.to_string(),
            language,
            problem_name: problem_name.to_string(),
            overall_oks,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.problem_name)
    }
}
