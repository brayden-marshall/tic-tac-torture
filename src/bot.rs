use super::{Board, EMPTY_SQUARE};

struct Position {
    row: usize,
    col: usize,
}

// chooses a move based on the perfect strategy detailed at:
// https://en.wikipedia.org/wiki/Tic-tac-toe#Strategy
pub fn make_move(player: char, board: &mut Board) {

    let functions = 
        [win, block, fork, block_fork, center, opposite_corner,
         empty_corner, empty_side];

    for func in functions.iter() {
        if let Some(p) = func(player, board) {
            board[p.row][p.col] = player;
            return;
        }
    }

    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == EMPTY_SQUARE {
                board[i][j] = player;
                return;
            }
            /*
            if let None = board[i][j] {
                board[i][j] = Some(player);
                return;
            }
            */
        }
    }
}

fn win(player: char, board: &Board) -> Option<Position> {
    None
}

fn block(player: char, board: &Board) -> Option<Position> {
    None
}

fn fork(player: char, board: &Board) -> Option<Position> {
    None
}

fn block_fork(player: char, board: &Board) -> Option<Position> {
    None
}

fn center(player: char, board: &Board) -> Option<Position> {
    None
}

fn opposite_corner(player: char, board: &Board) -> Option<Position> {
    None
}

fn empty_corner(player: char, board: &Board) -> Option<Position> {
    None
}

fn empty_side(player: char, board: &Board) -> Option<Position> {
    None
}
