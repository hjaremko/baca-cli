use crate::error::Error;
use crate::model::Task;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new(tasks: Vec<Task>) -> Self {
        Tasks { tasks }
    }

    pub fn print(&self) {
        for task in &self.tasks {
            let s = format!(
                "● Id: {} - {} - {} OK",
                task.id, task.problem_name, task.overall_oks
            )
            .bold();
            println!("{}", s);
        }
    }

    pub fn get_by_id(&self, task_id: &str) -> Result<&Task, Error> {
        self.tasks
            .iter()
            .find(|x| x.id == task_id)
            .ok_or_else(|| Error::InvalidTaskId(task_id.to_string()))
    }
}
