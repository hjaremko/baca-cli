use crate::model::SubmitStatus;

#[derive(Debug, PartialEq, Clone)]
pub struct TestResults {
    pub name: String,
    pub status: SubmitStatus,
}
