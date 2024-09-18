use crate::enums::Language;

use super::{Problem, Solution};

#[derive(Clone)]
pub struct Record {
    pub id: u16,
    pub title: String,
    pub difficulty: String,
    pub url: String,
    pub solutions: Vec<Language>,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let solutions = self
            .solutions
            .iter()
            .map(|language| language.to_readme(&self.id))
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "| {}. [{}]({}) | {} | {} |",
            self.id, self.title, self.url, solutions, self.difficulty
        )
    }
}

impl From<(Problem, Solution)> for Record {
    fn from((problem, solution): (Problem, Solution)) -> Self {
        Record {
            id: problem.id,
            title: problem.title,
            difficulty: problem.difficulty,
            url: problem.url,
            solutions: solution.languages,
        }
    }
}

impl From<&str> for Record {
    fn from(line: &str) -> Self {
        let chunks = line.split('|').collect::<Vec<&str>>();

        let difficulty = chunks[chunks.len() - 2].trim().to_string();

        let (id_raw, block) = chunks[1].trim().split_once(". ").unwrap();
        let id = id_raw.parse::<u16>().unwrap();
        let (name, url) = block.split_once('(').unwrap();
        let title = name[1..name.len() - 1].to_string();
        let url = url[0..url.len() - 1].to_string();

        let solutions_raw = chunks[2].trim();
        let mut solutions = Vec::new();
        for solution in solutions_raw.split(", ") {
            let language_raw = solution.split_once('(').unwrap().0;
            solutions.push(Language::from(&language_raw[1..language_raw.len() - 1]));
        }

        Record {
            id,
            title,
            difficulty,
            url,
            solutions,
        }
    }
}
