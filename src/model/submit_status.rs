use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum SubmitStatus {
    Ok,
    WrongAnswer,
    TimeExceeded,
    NoHeader
}

impl FromStr for SubmitStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<SubmitStatus, Self::Err> {
        match input {
            "program zaakceptowany" => Ok(SubmitStatus::Ok),
            "zĹ\\x82a odpowiedz" => Ok(SubmitStatus::WrongAnswer),
            "przekroczony czas" => Ok(SubmitStatus::TimeExceeded),
            "brak nagĹ\\x82Ăłwka" => Ok(SubmitStatus::NoHeader),
            _ => Ok(SubmitStatus::WrongAnswer), // todo: different colors for different statuses
                                                // _ => Err(()),
        }
    }
}
