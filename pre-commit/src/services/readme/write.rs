use std::fs;

use crate::constants;

pub fn write(table: String, counters: String) {
    let readme = fs::read_to_string(constants::README_PATH).expect("Unable to read file");

    let new_readme = format!(
        "{}{}{}{}",
        readme.split(constants::COUNTERS_START).collect::<Vec<&str>>()[0],
        counters,
        table,
        readme.split(constants::TABLE_END).collect::<Vec<&str>>()[1]
    );

    fs::write(constants::README_PATH, new_readme).expect("Unable to write file");
}
