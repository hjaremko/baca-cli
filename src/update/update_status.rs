use crate::update::BacaRelease;

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateStatus {
    NoUpdates,
    Update(BacaRelease),
}
