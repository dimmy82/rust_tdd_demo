#![allow(warnings)]

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

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
//    println!("sum of passed records: {:?}", sum_of_passed_records(Box::new(RecordApi::new())))
}

#[cfg(test)]
mod tests {
    use mockers::Scenario;
    use super::*;
    use domain::TRecords;

    #[test]
    fn test_sum_of_passed_records_succeed() {}
}