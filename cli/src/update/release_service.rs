use api::error::Result;
use crate::update::BacaRelease;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ReleaseService {
    fn get_last_release(&self) -> Result<BacaRelease>;
}
