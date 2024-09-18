use std::{collections::HashMap, fs};

use crate::{constants, data::Solution, enums::Language};

pub fn get_solutions() -> Vec<Solution> {
    let mut solutions: HashMap<u16, Vec<Language>> = HashMap::new();

    for path in constants::SOLUTION_PATHS {
        let files = fs::read_dir(path).unwrap();
        for file in files {
            let file = file.unwrap().path();
            if file.is_file() {
                let name = file.file_name().unwrap().to_str().unwrap();
                if name.starts_with('p') {
                    let id = name[1..5].parse::<u16>().unwrap();
                    let language = Language::from_extension(&name[6..name.len()]);
                    solutions.entry(id).or_default().push(language);
                }
            }
        }
    }

    solutions
        .iter()
        .map(|(&id, languages)| Solution::new(id, languages.to_vec()))
        .collect::<Vec<Solution>>()
}
