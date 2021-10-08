use crate::model::SubmitStatus;

#[derive(Debug, PartialEq)]
pub struct TestResults {
    pub name: String,
    pub status: SubmitStatus,
}
