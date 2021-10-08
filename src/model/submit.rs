use crate::model::submit_status::SubmitStatus;
use crate::model::TestResults;
use colored::*;

#[derive(Debug, PartialEq)]
pub struct Submit {
    // problem: Problem,
    pub status: SubmitStatus,
    pub points: f32,
    pub lateness: Option<i32>,
    pub accepted: i32,
    pub size: i32,
    pub timestamp: String,
    pub language: String,
    pub id: String,
    pub max_points: Option<i32>,
    pub problem_name: String,
    pub link: String,
    pub test_results: Option<Vec<TestResults>>,
}

impl Submit {
    // todo: ctor
    pub fn print_with_tests(&self) {
        self.print();

        if self.test_results.is_none() {
            return;
        }

        let test_results = self.test_results.as_ref().unwrap();

        let first = test_results.first().unwrap();
        let first_str = format!(" ── {} - {:?}", first.name, first.status);
        let first_str = add_emoji(&first_str, &first.status);
        let first_str = apply_color_according_to_status(&first_str, &first.status);
        println!("{}", first_str);

        if test_results.len() > 2 {
            let mid = test_results;
            let mid = &mid[1..mid.len() - 1];
            for test in mid {
                let test_str = format!(" ── {} - {:?}", test.name, test.status);
                let test_str = add_emoji(&test_str, &test.status);
                let test_str = apply_color_according_to_status(&test_str, &test.status);
                println!("{}", test_str);
            }
        }

        if test_results.len() > 3 {
            let last = test_results.last().unwrap();
            let last_str = format!(" ── {} - {:?}", last.name, last.status);
            let last_str = add_emoji(&last_str, &last.status);
            let last_str = apply_color_according_to_status(&last_str, &last.status);
            println!("{}", last_str);
        }
    }

    pub fn print(&self) {
        let header_line = self.make_header_line();
        let status_line = self.make_status_line();
        let link_line = self.make_link_line();

        let submit_info = format!("{}\n{}\n{}", header_line, status_line, link_line);
        let submit_info = apply_color_according_to_status(&submit_info, &self.status);

        println!("{}", submit_info);
    }

    fn make_link_line(&self) -> String {
        format!("└─── {}", self.link)
    }

    fn make_status_line(&self) -> String {
        match self.max_points {
            None => format!(
                "├─── {}% - {} pts - {:?}",
                self.accepted, self.points, self.status
            ),
            Some(max) => format!(
                "├─── {}% - {}/{} pts - {:?}",
                self.accepted, self.points, max, self.status
            ),
        }
    }

    fn make_header_line(&self) -> String {
        format!(
            "● {} - {} - {} - submit {}",
            self.problem_name, self.language, self.timestamp, self.id
        )
    }
}

fn add_emoji(str: &str, status: &SubmitStatus) -> String {
    match status {
        SubmitStatus::Ok => format!(" ✔️{}", str),
        _ => format!(" ❌{}", str),
    }
}

fn apply_color_according_to_status(str: &str, status: &SubmitStatus) -> ColoredString {
    match status {
        SubmitStatus::Ok => str.green().bold(),
        SubmitStatus::Processing => str.bright_yellow().bold(),
        SubmitStatus::InQueue => str.bright_yellow().bold(),
        SubmitStatus::WrongAnswer => str.yellow().bold(),
        SubmitStatus::TimeExceeded => str.yellow().bold(),
        SubmitStatus::CompileError => str.yellow().bold(),
        SubmitStatus::NoHeader => str.blue().bold(),
        SubmitStatus::RealTimeExceeded => str.yellow().bold(),
        SubmitStatus::ManuallyRejected => str.magenta().bold(),
        SubmitStatus::RuntimeError => str.yellow().bold(),
        SubmitStatus::InternalError => str.red().bold(),
        SubmitStatus::OutputSizeExceeded => str.yellow().bold(),
    }
}

// todo: enable when ctor is done
// #[cfg(test)]
// mod submit_print_tests {
//     use crate::model::Submit;
//
//     #[test]
//     fn correct_submit() {
//         let s = Submit::new(
//             "1234".to_string(),
//             "Kupcy i piraci",
//             100.0,
//             "https://baca.ii.uj.edu.pl/so2018/#SubmitDetails/1234",
//         );
//
//         s.print();
//     }
//
//     #[test]
//     fn wrong_answer_submit() {
//         let s = Submit::new(
//             "1234".to_string(),
//             "Ada. Szkółka leśna",
//             45.0,
//             "https://baca.ii.uj.edu.pl/so2018/#SubmitDetails/1234",
//         );
//
//         s.print();
//     }
// }
