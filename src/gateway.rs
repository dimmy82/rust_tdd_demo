use ::domain::TRecords;
#[cfg(test)]
use mockers_derive::mocked;
use ::domain::Records;

#[cfg(not(test))]
const BASE_URL: &str = "http://localhost:3200";
#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;

#[cfg_attr(test, mocked)]
pub trait TRecordApi {
    fn all_records(&self) -> Box<TRecords>;
}

pub struct RecordApi;

impl RecordApi {
    pub fn new() -> RecordApi {
        RecordApi {}
    }
}

impl TRecordApi for RecordApi {
    fn all_records(&self) -> Box<TRecords> {
        Box::new(reqwest::get(&format!("{}{}", BASE_URL, "/v1/records")).unwrap().json::<Records>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use super::*;
    use ::domain::Record;

    #[test]
    fn test_record_api_all_records_succeed() {
        let _api_mock = mock("GET", "/v1/records")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"list": [{ "score": 100 }, { "score": 90 }]}"#)
            .create();
        let records = RecordApi::new().all_records();
        assert_eq!(records.to_self(),
                   &Records {
                       list: vec![
                           Record {
                               score: 100
                           },
                           Record {
                               score: 90
                           }
                       ]
                   });
    }
}