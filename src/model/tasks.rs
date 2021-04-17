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
}

// todo: fetch all data
#[derive(Debug)]
pub struct Task {
    pub id: String,
    // pub language: String,
    pub problem_name: String,
    pub overall_oks: i32,
}
