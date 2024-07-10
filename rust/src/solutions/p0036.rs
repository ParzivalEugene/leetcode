pub struct Solution;

impl Solution {
    fn check_group(group: &Vec<char>) -> bool {
        let block = group
            .iter()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
        block.len() == block.iter().collect::<std::collections::HashSet<_>>().len()
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                let mut square: Vec<char> = Vec::new();
                for i in 0..3 {
                    square.append(&mut board[3 * y + i][3 * x..3 * x + 3].to_vec());
                }
                if !Self::check_group(&square) {
                    return false;
                }
            }
        }

        for row in &board {
            if !Self::check_group(row) {
                return false;
            }
        }

        for x in 0..9 {
            let mut column: Vec<char> = Vec::new();
            for y in 0..9 {
                column.push(board[y][x])
            }
            if !Self::check_group(&column) {
                return false;
            }
        }

        true
    }
}
