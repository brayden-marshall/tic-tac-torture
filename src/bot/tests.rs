use super::*;
use super::super::*;
use std::collections::HashSet;

use PlayerKind::*;

static EMP: Option<PlayerKind> = None;
static P_X: Option<PlayerKind> = Some(PlayerX);
static P_O: Option<PlayerKind> = Some(PlayerO);

/// # Arguments
/// * `name` - the identifier of the test function being made
/// * `func` - the name of the function being tested
/// * `p` - a variable of type char, indicating the player we are 
/// passing to the check function
/// * `board` - a variable of type Board, used as the board's state
/// * `expected` - a variable of type Position, the expected 
/// return value of `func`
macro_rules! board_test_some {
    ($name:ident, $func:ident, $p:expr, $board:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let win_position = match $func($p, &$board) {
                Some(p) => p,
                None => panic!("Function should return Some(_), found None"),
            };

            //eprintln!("win_position: {:?}, expected: {:?}",
            //          win_position, $expected);
            assert_eq!(win_position.row, $expected.row);
            assert_eq!(win_position.col, $expected.col);
        }
    }
}

/// Works similar to `board_test_some!`, but expects `$func` to return None,
/// indicating that no matching move was found
macro_rules! board_test_none {
    ($name:ident, $func:ident, $p:expr, $board:expr) => {
        #[test]
        fn $name() {
            assert!(if let None = $func($p, &$board) { true } else { false });
        }
    }
}

// testing win()
board_test_none!(none_win_test, win, PlayerX,
                 [[P_X, EMP, P_O],
                  [EMP, P_O, EMP],
                  [EMP, P_X, EMP]]);

board_test_some!(vertical_win_test, win, PlayerX,
                 [[P_X, EMP, P_O],
                  [P_X, EMP, EMP],
                  [EMP, EMP, P_O]],
                 Position {row: 2, col: 0});

board_test_some!(horizontal_win_test, win, PlayerX,
                 [[P_X, EMP, P_O],
                  [P_O, EMP, P_O],
                  [P_X, P_X, EMP]],
                 Position {row: 2, col: 2});

board_test_some!(diagonal_win_test, win, PlayerX,
                 [[P_X, P_O, EMP],
                  [P_O, EMP, EMP],
                  [EMP, EMP, P_X]],
                 Position {row: 1, col: 1});

// testing block()
board_test_none!(none_block_test, block, PlayerX, 
                 [[EMP, P_X, P_O],
                  [P_O, P_X, EMP],
                  [EMP, EMP, EMP]]);

board_test_some!(some_block_test, block, PlayerX,
                 [[EMP, EMP, P_O],
                  [EMP, P_X, P_O],
                  [P_X, EMP, EMP]],
                 Position {row: 2, col: 2});

// testing fork()
board_test_none!(none_fork_test, fork, PlayerX,
                 [[P_X, EMP, P_O],
                  [EMP, EMP, EMP],
                  [P_X, EMP, P_O]]);

board_test_some!(corner_fork_test, fork, PlayerX,
                 [[P_X, EMP, EMP],
                  [EMP, P_O, EMP],
                  [P_O, EMP, P_X]],
                 Position {row: 0, col: 2});

board_test_some!(center_fork_test, fork, PlayerX,
                 [[P_X, P_X, EMP],
                  [EMP, EMP, P_O],
                  [P_O, EMP, EMP]],
                 Position {row: 1, col: 1});

board_test_some!(side_fork_test, fork, PlayerX,
                 [[P_X, EMP, P_O],
                  [EMP, EMP, P_X],
                  [EMP, EMP, P_O]],
                 Position {row: 1, col: 0});

// testing block_fork()
board_test_none!(none_block_fork_test, block_fork, PlayerX,
                 [[P_X, EMP, P_O],
                  [EMP, EMP, EMP],
                  [P_X, EMP, P_O]]);

board_test_some!(corner_block_fork_test, block_fork, PlayerX,
                 [[P_O, EMP, P_X],
                  [EMP, P_X, EMP],
                  [EMP, EMP, P_O]],
                 Position {row: 2, col: 0});

board_test_some!(double_block_fork_test1, block_fork, PlayerO,
                 [[P_X, EMP, EMP],
                  [EMP, P_O, EMP],
                  [EMP, EMP, P_X]],
                 Position {row: 0, col: 1});

// testing opposite_corner()
board_test_some!(opposite_corner_test, opposite_corner, PlayerX,
                 [[P_O, EMP, EMP],
                  [EMP, EMP, EMP],
                  [EMP, EMP, EMP]],
                 Position {row: 2, col: 2});

// testing empty_corner()
board_test_none!(none_empty_corner_test, empty_corner, PlayerX,
                 [[P_O, EMP, P_X],
                  [EMP, EMP, EMP],
                  [P_O, EMP, P_X]]);

board_test_some!(some_empty_corner_test, empty_corner, PlayerX,
                 [[EMP, P_O, EMP],
                  [P_O, P_X, P_O],
                  [EMP, P_X, EMP]],
                 Position {row: 0, col: 0});

// testing empty_side()
board_test_none!(none_empty_side_test, empty_side, PlayerX,
                 [[EMP, P_O, EMP],
                  [P_O, P_X, P_O],
                  [EMP, P_X, EMP]]);

board_test_some!(some_empty_side_test, empty_side, PlayerX,
                 [[P_O, EMP, P_X],
                  [EMP, EMP, EMP],
                  [P_O, EMP, P_X]],
                 Position {row: 1, col: 0});

#[test]
fn valid_move_count_test() {
    // winning move
    let mut board: Board = 
        [[P_X, EMP, EMP],
         [P_O, P_O, EMP],
         [P_X, EMP, EMP]];
    assert_eq!(valid_move_count(PlayerX, &board, win), 0);

    board =
        [[P_X, P_O, P_X],
         [P_O, EMP, P_O],
         [P_X, EMP, EMP]];
    assert_eq!(valid_move_count(PlayerX, &board, win), 1);

    board =
        [[P_X, EMP, P_X],
         [EMP, P_O, EMP],
         [P_X, EMP, P_O]];
    assert_eq!(valid_move_count(PlayerX, &board, win), 2);

    // forking move
    board =
        [[P_X, P_O, EMP],
         [P_X, P_O, EMP],
         [P_O, EMP, P_X]];
    assert_eq!(valid_move_count(PlayerX, &board, fork), 0);

    board =
        [[P_X, P_O, EMP],
         [EMP, EMP, P_X],
         [EMP, EMP, P_O]];
    assert_eq!(valid_move_count(PlayerX, &board, fork), 1);

    board =
        [[P_X, EMP, EMP],
         [EMP, P_O, EMP],
         [EMP, EMP, P_X]];
    assert_eq!(valid_move_count(PlayerX, &board, fork), 2);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct GameState {
    last_player: PlayerKind,
    board: Board,
}

fn brute_force_helper(current_player: PlayerKind, bot_player: PlayerKind, 
                      mut board: Board, failed_games: &mut HashSet<GameState>) {
    // base case: game is tied
    if is_full(&board) {
        return;
    }

    // base case: the previous move won the game
    if has_won(opposite_player(current_player), &board) {
        // record if the non-bot player has won (test failure case)
        if opposite_player(current_player) != bot_player {
            failed_games.insert(GameState {
                last_player: opposite_player(current_player),
                board: board,
            });
        }
        return;
    }

    // if it's the bot's turn use it's function to make a move
    if current_player == bot_player {
        let (row, col) = get_move(current_player, &mut board);
        board[row][col] = Some(current_player);
        brute_force_helper(opposite_player(current_player), bot_player, 
                           board, failed_games);
    // if it's the non-bot's turn, recursively call for every possible move
    } else {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if let None = board[i][j] {
                    let mut board_copy = board.clone();
                    board_copy[i][j] = Some(current_player);
                    brute_force_helper(opposite_player(current_player), bot_player, 
                                       board_copy, failed_games);
                }
            }
        }
    }
}

/// This test checks every possible game scenario and ensures
/// that the bot's move will either lead to it winning or 
/// tieing the game.
#[test]
#[ignore]
fn brute_force_everything() {
    let mut failed_games = HashSet::new();
    // test for bot == 'X'
    brute_force_helper(PlayerX, PlayerX, [[None; 3]; 3], &mut failed_games);

    // test for bot == 'O'
    brute_force_helper(PlayerX, PlayerO, [[None; 3]; 3], &mut failed_games);
    assert!(failed_games.is_empty(), "Failed game states were:\n {:?}", failed_games);
}
