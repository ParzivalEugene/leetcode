use crate::{
    data::{Record, Solution},
    services::leetcode,
};

pub fn update_solutions(mut records: Vec<Record>, solutions: Vec<Solution>) -> Vec<Record> {
    for solution in solutions {
        if let Some(record) = records.iter_mut().find(|record| record.id == solution.id) {
            record.solutions = solution.languages;
        } else {
            let problem = leetcode::problem_info(solution.id);
            records.push(Record::from((problem, solution)));
        }
    }

    records
}
