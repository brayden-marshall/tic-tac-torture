use super::{Board, EMPTY_SQUARE};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

// chooses a move based on the perfect strategy detailed at:
// https://en.wikipedia.org/wiki/Tic-tac-toe#Strategy
pub fn make_move(player: char, board: &mut Board) {

    // array of functions to check for a matching move
    // the order is very important to the correctness of the algorithm
    let functions = 
        [win, block, fork, block_fork, center, opposite_corner,
         empty_corner, empty_side];

    for func in functions.iter() {
        if let Some(p) = func(player, board) {
            board[p.row][p.col] = player;
            return;
        }
    }
}

fn opposite_player(player: char) -> char{
    if player == 'X' {
        return 'O';
    } else if player == 'O' {
        return 'X';
    } else {
        // it's okay to panic here, because it should only be caused by
        // a hard-coded programmer error
        panic!("Invalid input passed to opposite_player()");
    }
}

/// Checks for two instances of 'player' in a row in one line of the board
/// The line is specified by the starting row/column and the offsets to 
/// by which to increment them to create the line.
fn check_two_in_a_row(player: char, board: &Board, row: usize, col: usize, 
                      offset_row: i32, offset_col: i32) -> Option<Position> {
    // assert that row and col are in bounds
    assert!(row < board.len());
    assert!(col < board.len());

    let mut i = row as i32;
    let mut j = col as i32;

    // the position of the single empty square in the line we are checking
    let mut empty_position: Option<Position> = None;
    // the count of how many instances of 'player' we find
    let mut count: i32 = 0;

    for _ in 0..board.len() {
        let square = board[i as usize][j as usize];
        if square == EMPTY_SQUARE {
            empty_position = match empty_position {
                None => Some(Position {row: i as usize, col: j as usize}),
                // if Some(_), it means we have found 2 empty squares,
                // so we return
                Some(_) => return None,
            }
        } else if square == player {
            count += 1;
        }

        i += offset_row;
        j += offset_col;
    }

    if count == 2 {
        empty_position
    } else {
        None
    }
}

/// Returns the position of any winning move for `player`
fn win(player: char, board: &Board) -> Option<Position> {
    let mut position: Option<Position>;
    for i in 0..board.len() {
        // check horizontal
        position = check_two_in_a_row(player, board, i, 0, 0, 1);
        if let Some(_) = position {
            return position;
        }

        // check vertical
        position = check_two_in_a_row(player, board, 0, i, 1, 0);
        if let Some(_) = position {
            return position;
        }
    }

    // check diagonals
    position = check_two_in_a_row(player, board, 0, 0, 1, 1);
    if let Some(_) = position {
        return position;
    }

    position = check_two_in_a_row(player, board, 0, 2, 1, -1);
    if let Some(_) = position {
        return position;
    }
    None
}

/// Returns the position of any move that will block a win for
/// `player`'s opponent
fn block(player: char, board: &Board) -> Option<Position> {
    // returning win() for the opposite player
    return win(opposite_player(player), board);
}

/// Higher order function that tests how many valid moves exist for
/// the function `func`. It is used in the implementations of the
/// `fork` and `block_fork` functions.
fn valid_move_count<F>(player: char, board: &Board, func: F) -> u32
                    where F: Fn(char, &Board) -> Option<Position>  {
    let mut board_copy = board.clone();
    let mut move_position = func(player, &board_copy);

    match move_position {
        None => return 0,
        Some(p) => {
            // place the opposite player at move_position, and check
            // if another valid move exists
            board_copy[p.row][p.col] = opposite_player(player);
            move_position = func(player, &board_copy);
            match move_position {
                None => return 1,
                Some(_) => return 2,
            }
        }
    }
}

/// Returns the position of any forking move for `player`.
fn fork(player: char, board: &Board) -> Option<Position> {
    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == EMPTY_SQUARE {
                let mut board_copy = board.clone();
                board_copy[i][j] = player;
                if valid_move_count(player, &board_copy, win) == 2 {
                    return Some(Position {row: i as usize, col: j as usize});
                }
            }
        }
    }
    None
}

/// Returns the position of any move that will block `player`'s
/// opponent from creating a fork.
fn block_fork(player: char, board: &Board) -> Option<Position> {
    let forking_move_count = 
        valid_move_count(opposite_player(player), board, fork);
    match forking_move_count {
        1 => fork(opposite_player(player), board),
        2 => block_double_fork(player, board),
        _ => None,
    }
}

fn block_double_fork(player: char, board: &Board) -> Option<Position> {
    // create two in a row, unless blocking it causes the opponent to fork
    for i in 0..board.len() {
        for j in 0..board.len() {
            let mut board_copy = board.clone();
            board_copy[i][j] = player;
            // if playing in this square results in a possible win
            if valid_move_count(player, &board_copy, win) > 0 {
                // if the opposite player blocking doesn't result in a fork
                if let Some(block_pos) = block(opposite_player(player), &board_copy) {
                    if let Some(fork_pos) = fork(opposite_player(player), &board_copy) {
                        if block_pos != fork_pos {
                            return Some(Position {row: i, col: j});
                        }
                    }
                }
            }
        }
    }
    None
}

fn center(_player: char, board: &Board) -> Option<Position> {
    if board[1][1] == EMPTY_SQUARE {
        return Some(Position {row: 1, col: 1});
    }
    None
}

fn opposite_corner(player: char, board: &Board) -> Option<Position> {
    let corners = [[0, 0], [2, 0], [2, 2], [0, 2]];
    let opposite_player = opposite_player(player);

    for corner in corners.iter() {
        if board[corner[0]][corner[1]] == opposite_player &&
           board[2 - corner[0]][2 - corner[1]] == EMPTY_SQUARE {
               return Some(Position {row: 2 - corner[0], col: 2 - corner[1]});
        }
    }
    None
}

fn empty_corner(_player: char, board: &Board) -> Option<Position> {
    let corners = [[0, 0], [2, 0], [2, 2], [0, 2]];

    for corner in corners.iter() {
        if board[corner[0]][corner[1]] == EMPTY_SQUARE {
            return Some(Position {row: corner[0], col: corner[1]});
        }
    }
    None
}

fn empty_side(_player: char, board: &Board) -> Option<Position> {
    let sides = [[1, 0], [2, 1], [1, 2], [0, 1]];

    for side in sides.iter() {
        if board[side[0]][side[1]] == EMPTY_SQUARE {
            return Some(Position {row: side[0], col: side[1]});
        }
    }
    None
}
