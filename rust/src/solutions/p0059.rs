pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut result = vec![vec![0; n]; n];

        fn validp(mat: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
            let i = i as usize;
            let j = j as usize;
            i < mat.len() && j < mat.len() && mat[i][j] == 0
        }

        let (mut i, mut j) = (0, 0);
        let (mut di, mut dj) = (0, 1);

        for k in 1..=((n * n) as i32) {
            result[i as usize][j as usize] = k;
            if !validp(&result, i + di, j + dj) {
                (di, dj) = match (di, dj) {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }
            i += di;
            j += dj;
        }

        result
    }
}
