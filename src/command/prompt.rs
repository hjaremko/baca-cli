use crate::error;
use crate::model::Tasks;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
#[cfg(test)]
use mockall::{automock, predicate::*};
use tracing::info;

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

pub struct TaskChoice {
    available_tasks: Tasks,
}

impl TaskChoice {
    pub fn new(available_tasks: Tasks) -> Self {
        Self { available_tasks }
    }
}

impl Prompt for TaskChoice {
    fn interact(&self) -> error::Result<String> {
        let items = &self.available_tasks.tasks;

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(items)
            .with_prompt("Choose task:")
            .default(0)
            .interact()?;

        info!("Selection index: {}", selection);
        Ok(items[selection].id.clone())
    }
}
