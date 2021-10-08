use crate::model::submit_status::SubmitStatus;
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
}

impl Submit {
    // todo: ctor
    // todo: print_extended with tests
    pub fn print(&self) {
        let header_line = self.make_header_line();
        let status_line = self.make_status_line();
        let link_line = self.make_link_line();

        let submit_info = format!("{}\n{}\n{}", header_line, status_line, link_line);
        let submit_info = self.apply_color_according_to_status(submit_info);

        println!("{}", submit_info);
    }

    fn apply_color_according_to_status(&self, submit_info: String) -> ColoredString {
        match self.status {
            SubmitStatus::Ok => submit_info.green().bold(),
            SubmitStatus::Processing => submit_info.bright_yellow().bold(),
            SubmitStatus::InQueue => submit_info.bright_yellow().bold(),
            SubmitStatus::WrongAnswer => submit_info.yellow().bold(),
            SubmitStatus::TimeExceeded => submit_info.yellow().bold(),
            SubmitStatus::CompileError => submit_info.yellow().bold(),
            SubmitStatus::NoHeader => submit_info.blue().bold(),
            SubmitStatus::RealTimeExceeded => submit_info.yellow().bold(),
            SubmitStatus::ManuallyRejected => submit_info.magenta().bold(),
            SubmitStatus::RuntimeError => submit_info.yellow().bold(),
            SubmitStatus::InternalError => submit_info.red().bold(),
            SubmitStatus::OutputSizeExceeded => submit_info.yellow().bold(),
        }
    }

    fn make_link_line(&self) -> String {
        format!("└─── {}\n", self.link)
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
