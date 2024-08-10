use std::collections::HashSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut magic_squares = 0;

        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        for i in 0..grid.len() - 3 + 1 {
            for j in 0..grid[i].len() - 3 + 1 {
                let mut elements_set = HashSet::new();
                let mut valid = 1;
                for i1 in i..i+3 {
                    for j1 in j..j+3 {
                        elements_set.insert(grid[i1][j1]);
                        if grid[i1][j1] < 1 || grid[i1][j1] > 9 {
                            valid = 0;
                        }
                    }
                }
                if elements_set.len() < 9 || valid == 0 {
                    continue;
                }

                let row1 = grid[i][j] + grid[i][j+1] + grid[i][j+2];
                let row2 = grid[i+1][j] + grid[i+1][j+1] + grid[i+1][j+2];
                let row3 = grid[i+2][j] + grid[i+2][j+1] + grid[i+2][j+2];
                let col1 = grid[i][j] + grid[i+1][j] + grid[i+2][j];
                let col2 = grid[i][j+1] + grid[i+1][j+1] + grid[i+2][j+1];
                let col3 = grid[i][j+2] + grid[i+1][j+2] + grid[i+2][j+2];
                let diag1 = grid[i][j] + grid[i+1][j+1] + grid[i+2][j+2];
                let diag2 = grid[i+2][j] + grid[i+1][j+1] + grid[i][j+2];

                if row1 == row2 && row2 == row3 && row3 == col1 && col1 == col2
                && col2 == col3 && col3 == diag1 && diag1 == diag2 {
                    magic_squares += 1;
                }
            }
        }
        return magic_squares;
    }
}