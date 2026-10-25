
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];

                if val == '.' {
                    continue;
                }

                let bit = 1u16 << (val as u8 - b'1');
                let box_idx = (r / 3) * 3 + (c / 3);

                if (rows[r] & bit) != 0
                    || (cols[c] & bit) != 0
                    || (boxes[box_idx] & bit) != 0
                {
                    return false;
                }

                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box_idx] |= bit;
            }
        }

        true
    }
}
