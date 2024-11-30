use std::collections::{BTreeMap, HashSet};

use crate::{constants, data::Record, enums::Language};

pub fn update_counters(records: &Vec<Record>) -> String {
    let global_counter = records
        .iter()
        .map(|record| record.id)
        .collect::<HashSet<u16>>()
        .len();
    let mut counter: BTreeMap<Language, u32> = BTreeMap::new();

    for record in records {
        for language in record.solutions.clone() {
            *counter.entry(language).or_insert(0) += 1;
        }
    }

    let mut counters = Vec::new();
    counters.push(format!(
        "![all](https://img.shields.io/badge/solved-{}-blue?style=for-the-badge)",
        global_counter
    ));
    for (language, count) in counter {
        counters.push(language.to_badge(count));
    }

    format!(
        "{}\n{}\n{}\n",
        constants::COUNTERS_START,
        counters.join(" "),
        constants::COUNTERS_END
    )
}
