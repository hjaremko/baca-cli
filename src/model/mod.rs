mod results;
pub use self::results::Results;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum SubmitStatus {
    Ok,
    WrongAnswer,
    TimeExceeded,
}

impl FromStr for SubmitStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<SubmitStatus, Self::Err> {
        match input {
            "program zaakceptowany" => Ok(SubmitStatus::Ok),
            "zĹ\\x82a odpowiedz" => Ok(SubmitStatus::WrongAnswer),
            "przekroczony czas" => Ok(SubmitStatus::TimeExceeded),
            _ => Ok(SubmitStatus::WrongAnswer), // todo: different colors for different statuses
                                                // _ => Err(()),
        }
    }
}

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

    pub fn print(&self) {
        use colored::*;

        let submit_info = format!(
            "● {} - {} - {} - submit {}",
            self.problem_name, self.language, self.timestamp, self.id
        );

        let submit_info = match self.status {
            SubmitStatus::Ok => submit_info.green().bold(),
            SubmitStatus::WrongAnswer => submit_info.yellow().bold(),
            SubmitStatus::TimeExceeded => submit_info.cyan().bold(),
        };

        match self.max_points {
            None => println!(
                "{}\n├─── {}% - {} pkt - {:?}",
                submit_info, self.accepted, self.points, self.status
            ),
            Some(max) => println!(
                "{}\n├─── {}% - {}/{} pkt - {:?}",
                submit_info, self.accepted, self.points, max, self.status
            ),
        };
        println!("└─── {}\n", self.link);
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
