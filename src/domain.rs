#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait TRecords {
    fn to_self(&self) -> &Records;
    fn sum_of_passed(&self) -> Record;
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Records {
    pub list: Vec<Record>
}

impl TRecords for Records {
    fn to_self(&self) -> &Records {
        self
    }

    fn sum_of_passed(&self) -> Record {
        Record {
            score: self.list.iter()
                .filter(|record| record.score >= 60)
                .fold(0, |acc, record| acc + record.score)
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Record {
    pub score: i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_records_sum_of_passed() {
        assert_eq!(Records {
            list: vec![
                Record { score: 100 },
                Record { score: 30 },
                Record { score: 60 },
                Record { score: 59 },
            ]
        }.sum_of_passed(), Record { score: 160 })
    }
}