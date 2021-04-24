use crate::update::BacaRelease;

#[derive(Debug, PartialEq)]
pub enum UpdateStatus {
    NoUpdates,
    Update(BacaRelease),
}
