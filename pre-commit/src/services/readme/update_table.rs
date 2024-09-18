use crate::{constants, data::Record};

pub fn update_table(mut records: Vec<Record>) -> String {
    records.sort_by(|a, b| a.id.cmp(&b.id));

    let table = records
        .iter()
        .map(|record| record.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{}\n| Problem | Solution | Difficulty |\n|---|---|---|\n{}\n{}",
        constants::TABLE_START,
        table,
        constants::TABLE_END
    )
}
