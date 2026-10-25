
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];

                if val == '.' {
                    continue;
                }

                let box_idx = (r / 3) * 3 + (c / 3);

                if rows[r].contains(&val)
                    || cols[c].contains(&val)
                    || boxes[box_idx].contains(&val)
                {
                    return false;
                }

                rows[r].insert(val);
                cols[c].insert(val);
                boxes[box_idx].insert(val);
            }
        }

        true
    }
}
