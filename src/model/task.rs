use crate::baca::details::Language;

#[derive(Debug, Clone, PartialEq)]
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
