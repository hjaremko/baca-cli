use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum SubmitStatus {
    OK,
    WrongAnswer,
}

impl SubmitStatus {
    pub fn from_score(score: i32) -> Self {
        if score == 100 {
            return Self::OK;
        }

        Self::WrongAnswer
    }
}

impl FromStr for SubmitStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<SubmitStatus, Self::Err> {
        match input {
            "program zaakceptowany" => Ok(SubmitStatus::OK),
            "zĹ\\x82a odpowiedz" => Ok(SubmitStatus::WrongAnswer),
            _ => Ok(SubmitStatus::WrongAnswer), // todo: different colors for different statuses
                                                // _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Submit {
    // problem: Problem,
    id: String,
    problem_name: String,
    score: i32,
    status: SubmitStatus,
    link: String,
}

impl Submit {
    pub fn new(id: String, problem_name: &str, score: i32, link: &str) -> Self {
        Submit {
            id,
            problem_name: problem_name.to_string(),
            score,
            status: SubmitStatus::from_score(score),
            link: link.to_string(),
        }
    }

    pub fn print(&self) {
        use colored::*;

        let submit_info = format!(
            "● {}% - {} - submit {}",
            self.score, self.problem_name, self.id
        );

        let submit_info = match self.status {
            SubmitStatus::OK => submit_info.green().bold(),
            SubmitStatus::WrongAnswer => submit_info.yellow().bold(),
            _ => submit_info.bold(),
        };

        println!("{}\n└─── {}\n", submit_info, self.link);
    }
}

#[cfg(test)]
mod submit_print_tests {
    use crate::model::Submit;

    #[test]
    fn correct_submit() {
        let s = Submit::new(
            "1234".to_string(),
            "Kupcy i piraci",
            100,
            "https://baca.ii.uj.edu.pl/so2018/#SubmitDetails/1234",
        );

        s.print();
    }

    #[test]
    fn wrong_answer_submit() {
        let s = Submit::new(
            "1234".to_string(),
            "Ada. Szkółka leśna",
            45,
            "https://baca.ii.uj.edu.pl/so2018/#SubmitDetails/1234",
        );

        s.print();
    }
}
