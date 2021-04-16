pub enum RequestType {
    Results,
    SubmitDetails(String),
    Login(String, String),
}

impl RequestType {
    pub fn payload_template(&self) -> String {
        match self {
        RequestType::Results =>
            "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getAllSubmits|Z|1|2|3|4|1|5|1|".to_string(),
        RequestType::SubmitDetails(id) =>
            "7|0|5|{}|03D93DB883748ED9135F6A4744CFFA07|testerka.gwt.client.submits.SubmitsService|getSubmitDetails|I|1|2|3|4|1|5|".to_string() + id + "|",
        RequestType::Login(_, _) =>
            "7|0|7|{}|620F3CE7784C04B839FC8E10C6C4A753|testerka.gwt.client.acess.PrivilegesService|login|java.lang.String/2004016611|{}|{}|1|2|3|4|2|5|5|6|7|".to_string(),
    }
    }

    pub fn mapping(&self) -> String {
        match *self {
            RequestType::Results => "submits".to_string(),
            RequestType::SubmitDetails(_) => "submits".to_string(),
            RequestType::Login(_, _) => "privileges".to_string(),
        }
    }
}
