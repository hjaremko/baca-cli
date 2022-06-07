use crate::model::Language;
use crate::parse::deserialize;
use crate::parse::from_baca_output::FromBacaOutput;
use crate::workspace::ConnectionConfig;
use tracing::debug;

impl FromBacaOutput for Option<Language> {
    fn from_baca_output(_: &ConnectionConfig, data: &str) -> Self {
        let data = deserialize(data);
        debug!("Deserialized: {:?}", data);

        if let Ok(language) = data[4].replace('\\', "").parse::<Language>() {
            Some(language)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_allowed_languages() {
        let mock_connection = ConnectionConfig::default();
        let data = r#"//OK[0,5,4,2,3,0,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","id","nazwa"],0,7]"#;
        let expected = None;
        let actual = Option::<Language>::from_baca_output(&mock_connection, data);

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_allowed_language() {
        let mock_connection = ConnectionConfig::default();
        let data = r#"//OK[0,7,6,2,3,5,4,2,3,1,2,1,[\"testerka.gwt.client.tools.DataSource/1474249525\",\"[[Ljava.lang.String;/4182515373\",\"[Ljava.lang.String;/2600011424\",\"1\",\"C++\",\"id\",\"nazwa\"],0,7]"#;
        let expected = Some(Language::Cpp);
        let actual = Option::<Language>::from_baca_output(&mock_connection, data);

        assert_eq!(actual, expected);
    }
}
