use ::domain::TRecords;
#[cfg(test)]
use mockers_derive::mocked;
//use ::domain::Records;

#[cfg(not(test))]
const BASE_URL: &str = "http://localhost:3200";
#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;

#[cfg_attr(test, mocked)]
pub trait TRecordApi {
    fn all_records(&self) -> Box<TRecords>;
}
