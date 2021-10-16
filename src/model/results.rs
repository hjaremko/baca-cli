use crate::model::Submit;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Results {
    pub submits: Vec<Submit>,
}

impl Results {
    pub fn new(submits: Vec<Submit>) -> Self {
        Self { submits }
    }

    pub fn print(&self, amount: usize) {
        self.submits.iter().take(amount).for_each(|s| s.print());
    }

    pub fn filter_by_task(&self, task_name: &str) -> Results {
        Results {
            submits: self
                .submits
                .iter()
                .filter(|submit| submit.problem_name == task_name)
                .cloned()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::SubmitStatus;

    fn make_mock_submit(name: &str) -> Submit {
        Submit {
            status: SubmitStatus::Processing,
            points: 0.0,
            lateness: None,
            accepted: 0,
            size: 123,
            timestamp: "2002".to_string(),
            language: "Bash".to_string(),
            id: "".to_string(),
            max_points: None,
            problem_name: name.to_string(),
            link: "www.baca.pl".to_string(),
            test_results: None,
        }
    }

    #[test]
    fn filter_empty_results() {
        let expected = Results::default();
        let actual = Results::default().filter_by_task("1");

        assert_eq!(actual, expected);
    }

    #[test]
    fn filter_single_log_no_matches() {
        let data = Results::new(vec![make_mock_submit("1")]);
        let expected = Results::default();
        let actual = data.filter_by_task("2");

        assert_eq!(actual, expected);
    }

    #[test]
    fn filter_single_log_match() {
        let expected = Results::new(vec![make_mock_submit("1")]);
        let actual = expected.filter_by_task("1");

        assert_eq!(actual, expected);
    }

    #[test]
    fn filter() {
        let data = Results::new(vec![
            make_mock_submit("1"),
            make_mock_submit("2"),
            make_mock_submit("3"),
            make_mock_submit("1"),
            make_mock_submit("1"),
            make_mock_submit("1"),
            make_mock_submit("2"),
            make_mock_submit("2"),
            make_mock_submit("4"),
            make_mock_submit("1"),
        ]);
        let expected = Results::new(vec![
            make_mock_submit("1"),
            make_mock_submit("1"),
            make_mock_submit("1"),
            make_mock_submit("1"),
            make_mock_submit("1"),
        ]);
        let actual = data.filter_by_task("1");

        assert_eq!(actual, expected);
    }
}
