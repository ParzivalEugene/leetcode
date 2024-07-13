use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut mapper: HashMap<String, i32> = HashMap::new();
        let mut counter = 0;
        for row in &grid {
            *mapper
                .entry(
                    row.iter()
                        .map(|&i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )
                .or_insert(0) += 1;
        }
        for i in 0..grid.len() {
            let column = grid
                .iter()
                .map(|row| row[i].to_string())
                .collect::<Vec<String>>()
                .join(",");
            counter += mapper.get(&column).or(Some(&0)).unwrap();
        }
        counter
    }
}
