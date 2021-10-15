use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum SubmitStatus {
    Ok,
    InQueue,
    Processing,
    WrongAnswer,
    TimeExceeded,
    CompileError,
    NoHeader,
    RealTimeExceeded,
    ManuallyRejected,
    RuntimeError,
    InternalError,
    OutputSizeExceeded,
}

impl FromStr for SubmitStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<SubmitStatus, Self::Err> {
        match input {
            "program zaakceptowany" => Ok(SubmitStatus::Ok),
            "czekanie na przetworzenie" => Ok(SubmitStatus::Processing),
            "w kolejce" => Ok(SubmitStatus::InQueue),
            "zĹ\\x82a odpowiedz" => Ok(SubmitStatus::WrongAnswer),
            "przekroczony czas" => Ok(SubmitStatus::TimeExceeded),
            "brak nagĹ\\x82Ăłwka" => Ok(SubmitStatus::NoHeader),
            "bĹ\\x82Ä\\x85d kompilacji" => Ok(SubmitStatus::CompileError),
            "bĹ\\x82Ä\\x85d wykonania: przekroczony real time" => {
                Ok(SubmitStatus::RealTimeExceeded)
            }
            _ => Ok(from_partial_str(input)),
        }
    }
}

fn from_partial_str(input: &str) -> SubmitStatus {
    const POSSIBLE_STATUSES: [(&str, SubmitStatus); 11] = [
        ("zaakceptowany", SubmitStatus::Ok),
        ("przetworzenie", SubmitStatus::Processing),
        ("odpowiedz", SubmitStatus::WrongAnswer),
        ("czas", SubmitStatus::TimeExceeded),
        ("real time", SubmitStatus::RealTimeExceeded),
        ("brak", SubmitStatus::NoHeader),
        ("kompilacji", SubmitStatus::CompileError),
        ("wykonania", SubmitStatus::RuntimeError),
        ("odrzucone", SubmitStatus::ManuallyRejected),
        ("testerki", SubmitStatus::InternalError),
        ("wyjscia", SubmitStatus::OutputSizeExceeded),
    ];

    for (status_str, status) in POSSIBLE_STATUSES {
        if input.contains(status_str) {
            return status;
        }
    }

    SubmitStatus::WrongAnswer
}

#[cfg(test)]
mod tests {
    use super::*;

    mod real_strings {
        use super::*;

        #[test]
        fn accepted() {
            assert_eq!(
                SubmitStatus::from_str("program zaakceptowany").unwrap(),
                SubmitStatus::Ok
            );
        }

        #[test]
        fn in_queue() {
            assert_eq!(
                SubmitStatus::from_str("w kolejce").unwrap(),
                SubmitStatus::InQueue
            );
        }

        #[test]
        fn processing() {
            assert_eq!(
                SubmitStatus::from_str("czekanie na przetworzenie").unwrap(),
                SubmitStatus::Processing
            );
        }
        #[test]
        fn wrong_answer() {
            assert_eq!(
                SubmitStatus::from_str("zĹ\\x82a odpowiedz").unwrap(),
                SubmitStatus::WrongAnswer
            );
        }
        #[test]
        fn time_exceeded() {
            assert_eq!(
                SubmitStatus::from_str("przekroczony czas").unwrap(),
                SubmitStatus::TimeExceeded
            );
        }
        #[test]
        fn compile_error() {
            assert_eq!(
                SubmitStatus::from_str("bĹ\\x82Ä\\x85d kompilacji").unwrap(),
                SubmitStatus::CompileError
            );
        }
        #[test]
        fn no_header() {
            assert_eq!(
                SubmitStatus::from_str("brak nagĹ\\x82Ăłwka").unwrap(),
                SubmitStatus::NoHeader
            );
        }

        #[test]
        fn runtime_error() {
            assert_eq!(
                SubmitStatus::from_str("bĹ\\x82Ä\\x85d wykonania").unwrap(),
                SubmitStatus::RuntimeError
            );
        }

        // todo: find out the real string
        // #[test]
        // fn manually_rejected() {
        //     assert_eq!(
        //         SubmitStatus::from_str("recznie odrzucono").unwrap(),
        //         SubmitStatus::ManuallyRejected
        //     );
        // }

        #[test]
        fn real_time_exceeded() {
            assert_eq!(
                SubmitStatus::from_str("bĹ\\x82Ä\\x85d wykonania: przekroczony real time").unwrap(),
                SubmitStatus::RealTimeExceeded
            );
        }

        #[test]
        fn unknown() {
            assert_eq!(
                SubmitStatus::from_str("unknown status").unwrap(),
                SubmitStatus::WrongAnswer
            );
        }
    }

    mod corrected_ascii_strings {
        use super::*;

        #[test]
        fn accepted() {
            assert_eq!(
                SubmitStatus::from_str("program zaakceptowany").unwrap(),
                SubmitStatus::Ok
            );
        }

        #[test]
        fn in_queue() {
            assert_eq!(
                SubmitStatus::from_str("w kolejce").unwrap(),
                SubmitStatus::InQueue
            );
        }

        #[test]
        fn processing() {
            assert_eq!(
                SubmitStatus::from_str("czekanie na przetworzenie").unwrap(),
                SubmitStatus::Processing
            );
        }
        #[test]
        fn wrong_answer() {
            assert_eq!(
                SubmitStatus::from_str("zla odpowiedz").unwrap(),
                SubmitStatus::WrongAnswer
            );
        }
        #[test]
        fn time_exceeded() {
            assert_eq!(
                SubmitStatus::from_str("przekroczony czas").unwrap(),
                SubmitStatus::TimeExceeded
            );
        }
        #[test]
        fn compile_error() {
            assert_eq!(
                SubmitStatus::from_str("blad kompilacji").unwrap(),
                SubmitStatus::CompileError
            );
        }
        #[test]
        fn no_header() {
            assert_eq!(
                SubmitStatus::from_str("brak naglowka").unwrap(),
                SubmitStatus::NoHeader
            );
        }

        #[test]
        fn runtime_error() {
            assert_eq!(
                SubmitStatus::from_str("blad wykonania").unwrap(),
                SubmitStatus::RuntimeError
            );
        }

        #[test]
        fn manually_rejected() {
            assert_eq!(
                SubmitStatus::from_str("recznie odrzucone").unwrap(),
                SubmitStatus::ManuallyRejected
            );
        }

        #[test]
        fn real_time_exceeded() {
            assert_eq!(
                SubmitStatus::from_str("blad wykonania: przekroczony real time").unwrap(),
                SubmitStatus::RealTimeExceeded
            );
        }

        #[test]
        fn internal_error() {
            assert_eq!(
                SubmitStatus::from_str("blad wewnetrzny testerki").unwrap(),
                SubmitStatus::InternalError
            );
        }

        #[test]
        fn output_size_exceeded() {
            assert_eq!(
                SubmitStatus::from_str("przekroczony rozmiar wyjscia").unwrap(),
                SubmitStatus::OutputSizeExceeded
            );
        }
    }
}
