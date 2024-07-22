pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut new = vec![];
        let len = matrix.len();
        for i in 0..len {
            let mut slice = vec![];
            for j in 0..len {
                slice.push(matrix[len - 1 - j][i]);
            }
            new.push(slice);
        }
        *matrix = new;
    }
}
