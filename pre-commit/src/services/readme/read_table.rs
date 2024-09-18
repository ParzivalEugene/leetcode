use std::fs;

use crate::{constants, data::Record};

pub fn read_table() -> Vec<Record> {
    let readme = fs::read_to_string(constants::README_PATH)
        .expect("Unable to read file")
        .split(constants::TABLE_START)
        .collect::<Vec<&str>>()[1]
        .split(constants::TABLE_END)
        .collect::<Vec<&str>>()[0]
        .to_owned();

    let mut table = readme.split('\n').collect::<Vec<&str>>();
    table = table[3..table.len() - 1].to_vec();

    let records = table
        .iter()
        .map(|line| Record::from(*line))
        .collect::<Vec<Record>>();

    records
}
