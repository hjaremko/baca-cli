use crate::error;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Prompt {
    fn interact(&self) -> error::Result<String>;
}

pub struct Input(pub &'static str);

impl Prompt for Input {
    fn interact(&self) -> error::Result<String> {
        Ok(dialoguer::Input::<String>::new()
            .with_prompt(self.0)
            .interact()?)
    }
}

pub struct Password;

impl Prompt for Password {
    fn interact(&self) -> error::Result<String> {
        Ok(dialoguer::Password::new()
            .with_prompt("Password")
            .interact()?)
    }
}
