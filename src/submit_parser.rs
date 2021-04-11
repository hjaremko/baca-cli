use crate::baca::InstanceData;
use crate::model::Submit;
use regex::Regex;

pub struct SubmitParser {}

impl SubmitParser {
    // todo: refactor
    pub fn parse(id: &str, instance: &InstanceData, data: &str) -> Option<Submit> {
        // let raw_regex = r#"""#.to_owned() + id + r#"","(?P<lang>.*)","(?P<date>.*)","(?P<bytes>\d*)","(?P<score>\d*)","\d*","(?P<points>[\d\.]*)","(?P<status>.*)""#;
        let raw_regex = r#"""#.to_owned()
            + id
            + r#"","(?P<lang>.*)","(?P<date>.*)","(?P<bytes>\d*)","(?P<score>\d*)","\d*","(?P<points>[\d\.]*)""#;
        let raw_regex_100 = r#"""#.to_owned()
            + id
            + r#"","(?P<lang>.*)","(?P<date>.*)","(?P<bytes>\d*)","(?P<score>\d*)""#;

        tracing::debug!("{}", raw_regex);
        tracing::debug!("{}", raw_regex_100);

        let re = Regex::new(raw_regex.as_str()).unwrap();
        let re100 = Regex::new(raw_regex_100.as_str()).unwrap();
        let mat = re.find(data);
        let mat100 = re100.find(data);

        tracing::debug!("{:?}", mat);
        tracing::debug!("{:?}", mat100);

        if mat.is_none() && mat100.is_none() {
            tracing::error!("Parsing submit {} data failed!", id);
            return None;
        }

        let caps = if mat.is_some() {
            re.captures(data).unwrap()
        } else {
            re100.captures(data).unwrap()
        };
        tracing::info!("{}", &caps["lang"]);
        tracing::info!("{}", &caps["date"]);
        tracing::info!("{}", &caps["bytes"]);

        let score = &caps["score"];
        tracing::info!("{}", score);

        let name_regex = r#""czas","status","(?P<name>.*)""#;
        tracing::debug!("{}", name_regex);
        let re = Regex::new(name_regex).unwrap();
        let caps = re.captures(data).unwrap();
        let name = &caps["name"].split('"').collect::<Vec<&str>>()[0];
        tracing::info!("{}", name);

        return Some(Submit::new(
            id.to_string(),
            name,
            score.parse().unwrap(),
            (instance.make_url() + "/#SubmitDetails/" + id).as_str(),
        ));
    }
}

#[cfg(test)]
mod submit_parser_tests {
    use crate::baca::InstanceData;
    use crate::model::Submit;
    use crate::submit_parser::SubmitParser;

    #[test]
    fn correct_submit() {
        let baca: InstanceData = InstanceData {
            host: "baca".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let data = r#""404","czas","status","[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane","4","2020-04-01 02:12:39","2020-04-17 23:00:00","2020-04-30 23:00:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","1721","C++","2020-04-04 00:25:12","6251","100","4.00","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","cos/1","exp/1","f1/1","f2/1","operatory/1","sin/1","test0/1","test"],0,7]"#;
        let actual = SubmitParser::parse("1721", &baca, &data).unwrap();

        let expected = Submit::new(
            "1721".to_string(),
            r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#,
            100,
            "https://baca.ii.uj.edu.pl/baca/#SubmitDetails/1721",
        );

        actual.print();
        assert_eq!(actual, expected);
    }

    #[test]
    fn wrong_submit() {
        let baca: InstanceData = InstanceData {
            host: "baca".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let data = r#""252","program zaakceptowany","236","220","248","244","czas","status","I - Sznury koralików","10","2019-01-19 22:00:00","2019-01-26 22:00:00","2019-02-02 22:00:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","12932","C++","2019-01-20 12:30:15","9255","90","100","9.05","bĹ\x82Ä\x85d wykonania","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","0_strand/0_01","0_strand/1_1b","0_strand/2_1a","0_strand/3_2b","0_strand/4_2m","0_strand/5_2a","0_strand/6_mm","1_bead/0_01","1_bead/1_1b","1_bead/2_1a","1_bead/3_2b","1_bead/4_2m","1_bead/5_2a","1_bead/6_mm","2_link/lm","2_link/ls","2_link/sbl","3_unlink/sblu","4_delete/sbld","5_move/sblm","6_remove/sblr","test"],0,7]"#;
        let actual = SubmitParser::parse("12932", &baca, &data).unwrap();

        let expected = Submit::new(
            "12932".to_string(),
            "I - Sznury koralików",
            90,
            "https://baca.ii.uj.edu.pl/baca/#SubmitDetails/12932",
        );

        actual.print();
        assert_eq!(actual, expected);
    }
}
