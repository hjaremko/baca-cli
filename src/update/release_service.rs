use crate::update::BacaRelease;
use anyhow::Result;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ReleaseService {
    fn get_last_release(&self) -> Result<BacaRelease>;
}
