pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                if (0..m).contains(&(i + 2)) && (0..n).contains(&(j + 2)) {
                    if Solution::is_magic(&grid, i, j) {
                        res += 1
                    }
                }
            }
        }

        res
    }

    pub fn is_magic(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        let mut sum: i32 = Solution::col_sum(&grid, i, j);
        let mut set: HashSet<i32> = HashSet::new();

        for x in 0..3 {
            for y in 0..3 {
                if (1..=9).contains(&grid[i + x][j + y]) && !set.contains(&grid[i + x][j + y]) {
                    set.insert(grid[i + x][j + y]);
                } else {
                    return false;
                }
            }
        }

        for row in 0..3 {
            if sum != Solution::row_sum(&grid, i + row, j) {
                return false;
            }
        }

        for col in 0..3 {
            if sum != Solution::col_sum(&grid, i, j + col) {
                return false;
            }
        }

        if sum != Solution::diagonal_sum(&grid, i as i32, j as i32, true) {
            return false;
        }

        if sum != Solution::diagonal_sum(&grid, i as i32 + 2, j as i32, false) {
            return false;
        }

        true
    }

    pub fn row_sum(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        grid[i][j..j + 3].iter().sum()
    }

    pub fn col_sum(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut sum = 0;
        for x in i..i + 3 {
            sum += grid[x][j]
        }
        sum
    }

    pub fn diagonal_sum(grid: &Vec<Vec<i32>>, i: i32, j: i32, principal: bool) -> i32 {
        let mut sum: i32 = 0;
        let direction: i32 = if principal { 1 } else { -1 };

        for d in 0..3 {
            sum += grid[(i + direction * d) as usize][(j + d) as usize]
        }

        sum
    }
}
