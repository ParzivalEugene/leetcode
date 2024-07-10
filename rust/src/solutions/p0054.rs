pub struct Solution;

impl Solution {
    fn rotate_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rotated = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                rotated[matrix[0].len() - 1 - j][i] = matrix[i][j];
            }
        }
        rotated
    }

    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut spiral = matrix.remove(0);
        while !matrix.is_empty() {
            matrix = Self::rotate_matrix(&matrix);
            spiral.append(&mut matrix.remove(0))
        }
        spiral
    }
}
