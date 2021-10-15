use crate::model::{Language, Task, Tasks};
use crate::parse::deserialize;
use std::str::FromStr;
use tracing::debug;

impl FromStr for Tasks {
    type Err = crate::error::Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let data = deserialize(data);
        debug!("Deserialized: {:?}", data);

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

        debug!("Parsed tasks: {:?}", tasks);
        Ok(Tasks::new(tasks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Language::Unsupported;

    #[test]
    fn real_data() {
        let raw_data = r#"//OK[0,12,11,10,3,3,9,8,7,3,3,6,5,4,3,3,2,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","1","Metoda parametryzacji","12","2","Metoda parametryzacji torusów","4","id","nazwa","liczba OK"],0,7]"#;
        let actual = raw_data.parse::<Tasks>().unwrap();
        let expected = Tasks::new(vec![
            Task::new("1", Unsupported, "Metoda parametryzacji", 12),
            Task::new("2", Unsupported, "Metoda parametryzacji torusów", 4),
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_data() {
        let raw_data = crate::api::details::EMPTY_RESPONSE;
        let actual = raw_data.parse::<Tasks>().unwrap();
        let expected = Tasks::new(Vec::new());

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_response() {
        let raw_data = "//OK[0,[3,3,6,5,4,3, invalid ababa],0,7]";
        let actual = raw_data.parse::<Tasks>().unwrap();
        let expected = Tasks::new(Vec::new());

        assert_eq!(actual, expected);
    }
}
