use crate::model::SubmitStatus;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TestResults {
    pub name: String,
    pub status: SubmitStatus,
}
