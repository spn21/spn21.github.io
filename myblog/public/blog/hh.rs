

const N: usize = 9;

fn solve_sudoku(board: &mut [[u8; N]; N]) -> bool {

    for row in 0..N {
        for board in 0..N {
            if board[row][col] ==0 {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = 0;
                    }
                }

                return false;
            }
        }
    }

    true
}

fn is_valid(board: &[[u8]])