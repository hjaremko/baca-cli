use crate::model::{Results, Submit, Tasks};
use crate::{baca, workspace};
use colored::Colorize;

pub fn init(host: &str, login: &str, pass: &str) {
    let mut instance = workspace::InstanceData {
        host: host.to_string(),
        login: login.to_string(),
        password: pass.to_string(),
        permutation: baca::details::permutation(),
        cookie: "".to_string(),
    };

    if let Err(e) = workspace::initialize() {
        println!("Initializing error: {}", e);
        return; // todo: return error code or something
    }

    instance.cookie = baca::api::get_cookie(&instance);
    workspace::save(&instance);
}

pub fn details(submit_id: &str) {
    let instance = workspace::read();
    let submit = baca::api::get_submit_details(&instance, submit_id);
    let submit = Submit::parse(&instance, &submit);

    submit.print();
}

pub fn refresh() {
    let mut instance = workspace::read();
    instance.cookie = baca::api::get_cookie(&instance);
    workspace::save(&instance);
}

pub fn log(n: usize) {
    let instance = workspace::read();
    let results = baca::api::get_results(&instance);
    let results = Results::parse(&instance, &results);

    results.print(n);
}

pub fn tasks() {
    let instance = workspace::read();
    let tasks = baca::api::get_tasks(&instance);
    let tasks = Tasks::parse(&tasks);

    tasks.print();
}

pub fn submit(task_id: &str, file_path: &str) {
    let instance = workspace::read();
    let tasks = baca::api::get_tasks(&instance);
    let tasks = Tasks::parse(&tasks);
    let task = tasks.get_by_id(task_id);

    println!(
        "Submitting {} to task {}.",
        file_path.bright_yellow(),
        task.problem_name.bright_green()
    );

    if let Err(msg) = baca::api::submit(&instance, task_id, file_path) {
        println!("{}", msg.bright_red());
    } else {
        println!();
        log(1);
    }
}
