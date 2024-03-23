use core::fmt;

trait Connect4Action {
    fn preform(&self, game_board: &Connect4GameBoard) -> ();
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Yellow,
    Red,
    Empty,
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "Y"),
            Color::Red => write!(f, "R"),
            Color::Empty => write!(f, "E"),
        }
    }
}

pub struct Connect4GameBoard {
    pub board: [[Color; 7]; 6],
}

impl fmt::Debug for Connect4GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str: String = String::from("");
        for row in self.board.iter().rev() {
            //write!(f, "{}", row)
            for color in row {
                str.push_str(&color.to_string());
                str.push(' ');
            }
            str.push_str("\n")
        }
        write!(f, "{}", str)
    }
}

impl Connect4GameBoard {
    pub fn available_columns(&mut self) -> Vec<usize> {
        let mut avail_cols: Vec<usize> = Vec::new();

        for col in 0..7 {
            if self.board[5][col] == Color::Empty {
                avail_cols.push(col);
            }
        }

        return avail_cols;
    }

    pub fn insert_coin(&mut self, column: usize, color: Color) -> bool {
        for row in 0..6 {
            if self.board[row][column] == Color::Empty {
                self.board[row][column] = color;

                return self.check_is_connect_4(row, column, color);
            }
        }

        return true;
    }

    fn check_is_connect_4(&self, row: usize, col: usize, color: Color) -> bool {
        for row_offset in 0..5 {
            let row_pos = row + row_offset;

            if row_pos as i32 - 1 < 0 || row_pos as i32 - 2 < 0 || row_pos as i32 - 3 < 0 {
                continue;
            }

            if row + row_offset > 5 || row + row_offset - 3 > 5 {
                continue;
            }

            if self.board[row_pos][col] == color
                && self.board[row_pos - 1][col] == color
                && self.board[row_pos - 2][col] == color
                && self.board[row_pos - 3][col] == color
            {
                return true;
            };
        }

        for col_offset in 0..6 {
            let col_pos = col + col_offset;

            if col_pos as i32 - 1 < 0 || col_pos as i32 - 2 < 0 || col_pos as i32 - 3 < 0 {
                continue;
            }

            if col + col_offset > 6 || col + col_offset - 3 > 6 {
                continue;
            }

            if self.board[row][col_pos] == color
                && self.board[row][col_pos - 1] == color
                && self.board[row][col_pos - 2] == color
                && self.board[row][col_pos - 3] == color
            {
                return true;
            };
        }

        for diagonal_offset in 0..5 {
            let col_pos = col + diagonal_offset;
            if col_pos as i32 - 1 < 0 || col_pos as i32 - 2 < 0 || col_pos as i32 - 3 < 0 {
                continue;
            }

            let row_pos = row + diagonal_offset;
            if row_pos as i32 - 1 < 0 || row_pos as i32 - 2 < 0 || row_pos as i32 - 3 < 0 {
                continue;
            }

            if row + diagonal_offset > 5
                || row + diagonal_offset - 3 > 5
                || col + diagonal_offset > 5
                || col + diagonal_offset + 3 > 6
            {
                continue;
            }

            if self.board[row + diagonal_offset][col + diagonal_offset] == color
                && self.board[row + diagonal_offset - 1][col + diagonal_offset - 1] == color
                && self.board[row + diagonal_offset - 2][col + diagonal_offset - 2] == color
                && self.board[row + diagonal_offset - 3][col + diagonal_offset - 3] == color
            {
                return true;
            };
        }

        for diagonal_offset in 0..5 {
            if col < diagonal_offset
                || col + 1 < diagonal_offset
                || col + 2 < diagonal_offset
                || col + 3 < diagonal_offset
            {
                continue;
            }

            let row_pos = row + diagonal_offset;
            if row_pos as i32 - 1 < 0 || row_pos as i32 - 2 < 0 || row_pos as i32 - 3 < 0 {
                continue;
            }

            if row + diagonal_offset > 5
                || row + diagonal_offset - 3 > 5
                || col - diagonal_offset > 5
                || col + 3 - diagonal_offset > 6
            {
                continue;
            }

            if self.board[row + diagonal_offset][col - diagonal_offset] == color
                && self.board[row + diagonal_offset - 1][col + 1 - diagonal_offset] == color
                && self.board[row + diagonal_offset - 2][col + 2 - diagonal_offset] == color
                && self.board[row + diagonal_offset - 3][col + 3 - diagonal_offset] == color
            {
                return true;
            };
        }

        return false;
    }
}
