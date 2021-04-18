use crate::baca::details::Language;
use crate::model::{Task, Tasks};
use crate::parse::deserialize;

impl Tasks {
    pub fn parse(data: &str) -> Tasks {
        let data = deserialize(data);
        tracing::debug!("Deserialized: {:?}", data);

        let st: Vec<String> = data.iter().skip(3).map(|x| x.to_owned()).collect();

        let tasks: Vec<_> = st
            .chunks(5)
            .into_iter()
            .rev()
            .skip(1)
            .map(|raw| Task {
                id: raw[4].to_string(),
                language: Language::Unsupported,
                problem_name: raw[3].to_string(),
                overall_oks: raw[2].parse().unwrap(),
            })
            .collect();

        tracing::debug!("Parsed tasks: {:?}", tasks);
        Tasks::new(tasks)
    }
}
