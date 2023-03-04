use crate::model::SubmitStatus;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TestResults {
    pub name: String,
    pub status: SubmitStatus,
}
