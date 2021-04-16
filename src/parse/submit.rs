use crate::model::Submit;
use crate::model::SubmitStatus;
use crate::workspace::InstanceData;
use std::str::FromStr;

impl Submit {
    pub fn parse(instance: &InstanceData, data: &str) -> Submit {
        let data = Self::deserialize(data);
        tracing::debug!("Deserialized: {:?}", data);

        let st: Vec<_> = data
            .iter()
            .skip_while(|x| !x.contains("nazwa statusu"))
            .map(|x| x.replace("\"", ""))
            .collect();

        let offset = 10;
        return Submit {
            status: SubmitStatus::from_str(&*st[offset]).unwrap(),
            points: st[offset + 1].parse().unwrap(),
            lateness: Some(st[offset + 2].parse().unwrap()),
            accepted: st[offset + 3].parse().unwrap(),
            size: st[offset + 4].parse().unwrap(),
            timestamp: st[offset + 5].to_string(),
            language: st[offset + 6].to_string(),
            id: st[offset + 7].to_string(),
            max_points: Some(st[offset + 7 + 25].parse().unwrap()),
            problem_name: st[offset + 7 + 26].to_string(),
            link: instance.make_url() + "/#SubmitDetails/" + st[offset + 7].as_str(),
        };
    }

    fn deserialize(data: &str) -> Vec<String> {
        let data: String = data.chars().skip(5).take(data.len() - 13).collect();
        let data = data.split(',').collect::<Vec<&str>>();

        let encoded = data
            .iter()
            .take_while(|x| (**x).chars().all(|c| c.is_ascii_digit()))
            .collect::<Vec<&&str>>();

        let codes = data
            .iter()
            .skip(encoded.len())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let codes = codes.join(",");
        let codes = codes.split(",\"").collect::<Vec<&str>>();

        let mut deserialized = Vec::<String>::new();
        for l in encoded {
            let val = l.to_string().parse::<usize>().unwrap();

            if val == 0 {
                continue;
            }

            deserialized.push((*codes[val - 1]).to_string());
        }

        deserialized
    }
}

#[cfg(test)]
mod submit_parser_tests {
    use crate::model::Submit;
    use crate::model::SubmitStatus;
    use crate::workspace::InstanceData;

    //todo: test code wih sequence "asda","asdasd"

    #[test]
    fn correct_mn_parse_test() {
        let baca = InstanceData {
            host: "mn".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let raw = r#"//OK[0,12,50,2,5,7,49,2,5,7,48,2,5,7,47,2,5,7,46,2,5,4,4,3,0,45,44,43,42,41,40,39,38,8,5,7,37,36,36,35,34,33,32,8,5,1,4,3,31,0,30,29,28,27,26,25,24,23,22,9,5,21,20,19,18,17,16,15,14,13,9,5,1,4,3,0,0,12,11,2,5,7,10,2,5,7,9,2,5,7,8,2,5,7,6,2,5,4,4,3,2,1,["testerka.gwt.client.submits.SubmitDetailsModel/2564112456","logs","testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","532","program zaakceptowany","536","564","572","czas","status","[G] Funkcje sklejane","4","2020-05-13 07:39:59","2020-06-04 23:00:00","2020-06-15 23:00:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","Limit pamieci na kompilacje (MB)","Limit kodu zrodlowego (kB)","code","4334","C++","2020-05-17 18:53:09","1190","100","4.00","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","test0/0","test1/0","test2/0","test3/0","test"],0,7]"#;

        let actual = Submit::parse(&baca, raw);
        let expected = Submit {
            status: SubmitStatus::Ok,
            points: 4.0,
            lateness: Some(100),
            accepted: 100,
            size: 1190,
            timestamp: "2020-05-17 18:53:09".to_string(),
            language: "C++".to_string(),
            id: "4334".to_string(),
            max_points: Some(4),
            problem_name: "[G] Funkcje sklejane".to_string(),
            link: "https://baca.ii.uj.edu.pl/mn/#SubmitDetails/4334".to_string(),
        };

        actual.print();
        assert_eq!(actual, expected);
    }

    #[test]
    fn incorrect_17_mn_parse_test() {
        let baca = InstanceData {
            host: "mn".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let raw = r#"//OK[0,9,53,2,5,36,52,2,5,36,51,2,5,36,50,2,5,36,49,2,5,36,48,2,5,7,47,2,5,7,46,2,5,7,45,2,5,8,4,3,0,44,43,42,41,40,39,38,37,8,5,36,35,34,33,32,31,30,29,8,5,1,4,3,28,0,27,26,25,24,23,22,21,20,19,9,5,18,17,16,15,14,13,12,11,10,9,5,1,4,3,0,0,9,8,2,5,7,6,2,5,1,4,3,2,1,["testerka.gwt.client.submits.SubmitDetailsModel/2564112456","logs","testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","320","program zaakceptowany","czas","status","[E] Metoda SOR","4","2020-04-23 09:19:09","2020-05-11 23:00:00","2020-05-25 23:00:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","Limit pamieci na kompilacje (MB)","Limit kodu zrodlowego (kB)","code","3266","C++","2020-04-26 12:43:36","1970","17","100","0.67","przekroczony czas","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","testy/test1","testy/test2","testy/test3","testy/test4","testy/test5","testy/test6","testy/test7","testy/test8","test"],0,7]"#;

        let actual = Submit::parse(&baca, raw);
        let expected = Submit {
            status: SubmitStatus::TimeExceeded,
            points: 0.67,
            lateness: Some(100),
            accepted: 17,
            size: 1970,
            timestamp: "2020-04-26 12:43:36".to_string(),
            language: "C++".to_string(),
            id: "3266".to_string(),
            max_points: Some(4),
            problem_name: "[E] Metoda SOR".to_string(),
            link: "https://baca.ii.uj.edu.pl/mn/#SubmitDetails/3266".to_string(),
        };

        actual.print();
        assert_eq!(actual, expected);
    }

    #[test]
    fn incorrect_80_mp_parse_test() {
        let baca = InstanceData {
            host: "mp".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let raw = r#"//OK[0,9,54,2,5,7,53,2,5,46,52,2,5,7,51,2,5,46,50,2,5,7,49,2,5,46,48,2,5,36,47,2,5,46,45,2,5,8,4,3,0,44,43,42,41,40,39,38,37,8,5,36,35,34,33,32,31,30,29,8,5,1,4,3,28,0,27,26,25,24,23,22,21,20,19,9,5,18,17,16,15,14,13,12,11,10,9,5,1,4,3,0,0,9,8,2,5,7,6,2,5,1,4,3,2,1,["testerka.gwt.client.submits.SubmitDetailsModel/2564112456","logs","testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","496","program zaakceptowany","czas","status","P05","3","2019-04-11 12:00:24","2019-04-25 22:00:24","2019-05-02 22:00:24","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","Limit pamieci na kompilacje (MB)","Limit kodu zrodlowego (kB)","code","2484","Java","2019-04-12 23:54:34","1944","38","100","1.13","przekroczony czas","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","test_nsum/big_nums_iter","zĹ\x82a odpowiedz","test_nsum/big_nums_rec","test_nsum/big_powers_iter","test_nsum/big_powers_rec","test_nsum/jawny_test_iter","test_nsum/jawny_test_rec","test_nsum/simple_iter","test_nsum/simple_rec","test"],0,7]"#;

        let actual = Submit::parse(&baca, raw);
        let expected = Submit {
            status: SubmitStatus::TimeExceeded,
            points: 1.13,
            lateness: Some(100),
            accepted: 38,
            size: 1944,
            timestamp: "2019-04-12 23:54:34".to_string(),
            language: "Java".to_string(),
            id: "2484".to_string(),
            max_points: Some(3),
            problem_name: "P05".to_string(),
            link: "https://baca.ii.uj.edu.pl/mp/#SubmitDetails/2484".to_string(),
        };

        actual.print();
        assert_eq!(actual, expected);
    }

    #[test]
    fn correct_pn_parse_test() {
        let baca = InstanceData {
            host: "pn".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let raw = r#"//OK[0,12,72,71,70,4,5,7,69,58,68,4,5,7,59,67,66,4,5,7,65,63,64,4,5,7,59,63,62,4,5,7,61,58,60,4,5,7,59,58,57,4,5,6,4,3,0,56,55,54,53,52,51,50,49,8,5,7,48,47,47,46,45,44,43,8,5,1,4,3,42,0,41,40,39,38,37,36,35,34,33,9,5,32,31,30,29,28,28,27,26,25,9,5,1,4,3,0,24,23,22,21,20,19,6,5,18,17,16,15,14,13,6,5,1,4,3,0,12,11,2,5,7,6,2,5,7,10,2,5,7,9,2,5,7,8,2,5,7,8,2,5,7,6,2,5,6,4,3,2,1,["testerka.gwt.client.submits.SubmitDetailsModel/2564112456","logs","testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","60","program zaakceptowany","56","64","68","czas","status","nazwisko","Imie","Nazwisko","nick","grupa nr 1","Prowadzacy","login","imię","nazwisko","nick","grupa","prowadzący","OPT1: MinMax","2","2020-12-16 09:58:00","2021-01-24 23:30:00","122","30","1024","125","Nazwa zdania","Liczba punktow do zdobycia","Start zadania","Termin oddania","Koniec zdania","Limit pamieci (MB)","Limit czasu kompilacji (s)","Limit pamieci na kompilacje (MB)","Limit kodu zrodlowego (kB)","code","478","C++","2021-01-13 12:27:10","991","100","2.00","id","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","spoznienie (%)","punkty","nazwa statusu","0/0","768","10000","0_t/0","1200","1/0","924","1_t/0","1050","2/0","772","2_t/0","1000","test","time","limit czasu"],0,7]"#;

        let actual = Submit::parse(&baca, raw);
        let expected = Submit {
            status: SubmitStatus::Ok,
            points: 2.0,
            lateness: Some(100),
            accepted: 100,
            size: 991,
            timestamp: "2021-01-13 12:27:10".to_string(),
            language: "C++".to_string(),
            id: "478".to_string(),
            max_points: Some(2),
            problem_name: "OPT1: MinMax".to_string(),
            link: "https://baca.ii.uj.edu.pl/pn/#SubmitDetails/478".to_string(),
        };

        actual.print();
        assert_eq!(actual, expected);
    }
}
