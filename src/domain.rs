#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait TRecords {
    //    fn to_self(&self) -> &Records;
    fn sum_of_passed(&self) -> Record;
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Record {
    pub score: i32
}
