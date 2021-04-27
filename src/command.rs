use crate::baca::details::Language;
use crate::model::{Results, Submit, Tasks};
use crate::workspace::Workspace;
use crate::{baca, error, workspace};
use colored::Colorize;
use tracing::{debug, info};

pub fn init<T: Workspace>(workspace: T, host: &str, login: &str, pass: &str) -> error::Result<()> {
    info!("Initializing Baca workspace.");
    debug!("Host: {}", host);
    debug!("Login: {}", login);
    debug!("Password: {}", pass);

    let mut instance = workspace::InstanceData {
        host: host.to_string(),
        login: login.to_string(),
        password: pass.to_string(),
        permutation: baca::details::permutation(),
        cookie: "".to_string(),
    };

    let cleanup_directory = |e| match e {
        error::Error::WorkspaceAlreadyInitialized => e,
        _ => {
            workspace
                .remove_workspace()
                .expect("Cannot cleanup baca directory");
            e
        }
    };

    workspace.initialize().map_err(cleanup_directory)?;
    instance.cookie = baca::api::get_cookie(&instance).map_err(cleanup_directory)?;
    workspace
        .save_instance(&instance)
        .map_err(cleanup_directory)?;
    Ok(())
}

pub fn details<T: Workspace>(workspace: T, submit_id: &str) -> error::Result<()> {
    info!("Printing details for submit: {}", submit_id);

    let instance = workspace.read_instance()?;
    let submit = baca::api::get_submit_details(&instance, submit_id)?;
    let submit = Submit::parse(&instance, &submit);

    submit.print();
    Ok(())
}

pub fn refresh<T: Workspace>(workspace: T) -> error::Result<()> {
    info!("Refreshing Baca session.");
    let mut instance = workspace.read_instance()?;
    instance.cookie = baca::api::get_cookie(&instance)?;
    workspace.save_instance(&instance)?;

    println!("New session obtained.");
    Ok(())
}

pub fn log<T: Workspace>(workspace: T, n: usize) -> error::Result<()> {
    info!("Fetching {} logs.", n);
    let instance = workspace.read_instance()?;
    let results = baca::api::get_results(&instance)?;
    let results = Results::parse(&instance, &results);

    results.print(n);
    Ok(())
}

pub fn tasks<T: Workspace>(workspace: T) -> error::Result<()> {
    let instance = workspace.read_instance()?;
    let tasks = baca::api::get_tasks(&instance)?;
    let tasks = Tasks::parse(&tasks);

    tasks.print();
    Ok(())
}

pub fn submit<T: Workspace>(
    workspace: T,
    task_id: &str,
    file_path: &str,
    lang: &Language,
) -> error::Result<()> {
    let instance = workspace.read_instance()?;
    let tasks = baca::api::get_tasks(&instance)?;
    let tasks = Tasks::parse(&tasks);
    let mut task = tasks
        .get_by_id(task_id)
        .ok_or_else(|| error::Error::InvalidTaskId(task_id.to_string()))?
        .clone();
    task.language = *lang;

    println!(
        "Submitting {} to task {} ({}).",
        file_path.bright_yellow(),
        task.problem_name.bright_green(),
        task.language.to_string()
    );

    baca::api::submit(&instance, &task, file_path)?;
    println!();
    log(workspace, 1)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{InstanceData, MockWorkspace};

    fn make_mock_instance() -> InstanceData {
        InstanceData {
            host: "host".to_string(),
            login: "login".to_string(),
            password: "pass".to_string(),
            permutation: "perm".to_string(),
            cookie: "invalid".to_string(),
        }
    }
    // todo: later
    #[test]
    fn dummy_refresh_test() {
        let mut mock = MockWorkspace::new();
        mock.expect_read_instance()
            .returning(|| Ok(make_mock_instance()));

        let result = refresh(mock);
        assert!(result.is_err())
    }
}
