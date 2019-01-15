//#![allow(warnings)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[cfg(test)]
extern crate mockers;
#[cfg(test)]
extern crate mockers_derive;
#[cfg(test)]
extern crate mockito;

mod domain;
mod gateway;

use domain::Record;
use gateway::{TRecordApi, RecordApi};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("sum of passed records: {:?}", sum_of_passed_records(Box::new(RecordApi::new())))
}

fn sum_of_passed_records(record_api: Box<TRecordApi>) -> Record {
    record_api.all_records().sum_of_passed()
}

#[cfg(test)]
mod tests {
    use mockers::Scenario;
    use super::*;
    use domain::TRecords;

    #[test]
    fn test_sum_of_passed_records_succeed() {
        let scenario = Scenario::new();
        let record_api = scenario.create_mock_for::<TRecordApi>();
        let records = scenario.create_mock_for::<TRecords>();

        scenario.expect(records.sum_of_passed_call()
            .and_return(Record { score: 120 }));
        scenario.expect(record_api.all_records_call()
            .and_return(Box::new(records)));

        assert_eq!(sum_of_passed_records(Box::new(record_api)), Record { score: 120 });
    }
}