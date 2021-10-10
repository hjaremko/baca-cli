use crate::baca::details::Language;
use crate::error::Error;
use colored::Colorize;

pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn new(tasks: Vec<Task>) -> Self {
        Tasks { tasks }
    }

    pub fn print(&self) {
        for task in &self.tasks {
            let s = format!(
                "● {} - {} - {} OK",
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

// todo: fetch all data
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub language: Language,
    pub problem_name: String,
    pub overall_oks: i32,
}
