pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let (mut h_index, len) = (0, citations.len());
        for i in 0..len {
            let current_citation = citations[i];
            let potential_h_index = std::cmp::min(len - i, current_citation as usize);
            h_index = std::cmp::max(potential_h_index as i32, h_index)
        }
        h_index
    }
}
