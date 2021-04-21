use crate::baca::details::Language;
use crate::model::{Results, Submit, Tasks};
use crate::{baca, error, workspace};
use colored::Colorize;
use tracing::{debug, info};

pub fn init(host: &str, login: &str, pass: &str) -> error::Result<()> {
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

    workspace::initialize()?;
    instance.cookie = baca::api::get_cookie(&instance)?;
    workspace::save_instance(&instance)?;
    Ok(())
}

pub fn details(submit_id: &str) -> error::Result<()> {
    info!("Printing details for submit: {}", submit_id);

    let instance = workspace::read_instance()?;
    let submit = baca::api::get_submit_details(&instance, submit_id)?;
    let submit = Submit::parse(&instance, &submit);

    submit.print();
    Ok(())
}

pub fn refresh() -> error::Result<()> {
    info!("Refreshing Baca session.");
    let mut instance = workspace::read_instance()?;
    instance.cookie = baca::api::get_cookie(&instance)?;
    workspace::save_instance(&instance)?;

    println!("New session obtained.");
    Ok(())
}

pub fn log(n: usize) -> error::Result<()> {
    info!("Fetching {} logs.", n);
    let instance = workspace::read_instance()?;
    let results = baca::api::get_results(&instance);
    let results = Results::parse(&instance, &results);

    results.print(n);
    Ok(())
}

pub fn tasks() -> error::Result<()> {
    let instance = workspace::read_instance()?;
    let tasks = baca::api::get_tasks(&instance);
    let tasks = Tasks::parse(&tasks);

    tasks.print();
    Ok(())
}

pub fn submit(task_id: &str, file_path: &str, lang: &Language) -> error::Result<()> {
    let instance = workspace::read_instance()?;
    let tasks = baca::api::get_tasks(&instance);
    let tasks = Tasks::parse(&tasks);
    let mut task = tasks.get_by_id(task_id).clone();
    task.language = *lang;

    println!(
        "Submitting {} to task {} ({}).",
        file_path.bright_yellow(),
        task.problem_name.bright_green(),
        task.language.to_string()
    );

    if let Err(msg) = baca::api::submit(&instance, &task, file_path) {
        println!("{}", msg.bright_red());
    } else {
        println!();
        log(1)?;
    }

    Ok(())
}
