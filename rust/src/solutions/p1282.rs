pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut groups: std::collections::HashMap<i32, Vec<Vec<i32>>> =
            std::collections::HashMap::new();
        for (i, size) in group_sizes.iter().enumerate() {
            groups
                .entry(*size)
                .and_modify(|group| {
                    if group.last().unwrap().len() == *size as usize {
                        group.push(vec![i as i32]);
                    } else {
                        let last = group.len() - 1;
                        group[last].push(i as i32);
                    }
                })
                .or_insert(vec![vec![i as i32]]);
        }
        for group in groups.values() {
            res.extend(group.clone());
        }
        res
    }
}
