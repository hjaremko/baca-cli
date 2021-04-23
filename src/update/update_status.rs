use crate::update::BacaRelease;

#[derive(Debug, PartialEq)]
pub enum UpdateStatus {
    _NoUpdates,
    _Update(BacaRelease),
}
