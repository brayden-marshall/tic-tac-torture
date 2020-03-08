use super::*;
use super::super::*;
use std::collections::HashSet;

const EMP: char = EMPTY_SQUARE;

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
board_test_none!(none_win_test, win, 'X',
                 [['X', EMP, 'O'],
                  [EMP, 'O', EMP],
                  [EMP, 'X', EMP]]);

board_test_some!(vertical_win_test, win, 'X',
                 [['X', EMP, 'O'],
                  ['X', EMP, EMP],
                  [EMP, EMP, 'O']],
                 Position {row: 2, col: 0});

board_test_some!(horizontal_win_test, win, 'X',
                 [['X', EMP, 'O'],
                  ['O', EMP, 'O'],
                  ['X', 'X', EMP]],
                 Position {row: 2, col: 2});

board_test_some!(diagonal_win_test, win, 'X',
                 [['X', 'O', EMP],
                  ['O', EMP, EMP],
                  [EMP, EMP, 'X']],
                 Position {row: 1, col: 1});

// testing block()
board_test_none!(none_block_test, block, 'X', 
                 [[EMP, 'X', 'O'],
                  ['O', 'X', EMP],
                  [EMP, EMP, EMP]]);

board_test_some!(some_block_test, block, 'X',
                 [[EMP, EMP, 'O'],
                  [EMP, 'X', 'O'],
                  ['X', EMP, EMP]],
                 Position {row: 2, col: 2});

// testing fork()
board_test_none!(none_fork_test, fork, 'X',
                 [['X', EMP, 'O'],
                  [EMP, EMP, EMP],
                  ['X', EMP, 'O']]);

board_test_some!(corner_fork_test, fork, 'X',
                 [['X', EMP, EMP],
                  [EMP, 'O', EMP],
                  ['O', EMP, 'X']],
                 Position {row: 0, col: 2});

board_test_some!(center_fork_test, fork, 'X',
                 [['X', 'X', EMP],
                  [EMP, EMP, 'O'],
                  ['O', EMP, EMP]],
                 Position {row: 1, col: 1});

board_test_some!(side_fork_test, fork, 'X',
                 [['X', EMP, 'O'],
                  [EMP, EMP, 'X'],
                  [EMP, EMP, 'O']],
                 Position {row: 1, col: 0});

// testing block_fork()
board_test_none!(none_block_fork_test, block_fork, 'X',
                 [['X', EMP, 'O'],
                  [EMP, EMP, EMP],
                  ['X', EMP, 'O']]);

board_test_some!(corner_block_fork_test, block_fork, 'X',
                 [['O', EMP, 'X'],
                  [EMP, 'X', EMP],
                  [EMP, EMP, 'O']],
                 Position {row: 2, col: 0});

board_test_some!(double_block_fork_test1, block_fork, 'O',
                 [['X', EMP, EMP],
                  [EMP, 'O', EMP],
                  [EMP, EMP, 'X']],
                 Position {row: 0, col: 1});

// testing opposite_corner()
board_test_some!(opposite_corner_test, opposite_corner, 'X',
                 [['O', EMP, EMP],
                  [EMP, EMP, EMP],
                  [EMP, EMP, EMP]],
                 Position {row: 2, col: 2});

// testing empty_corner()
board_test_none!(none_empty_corner_test, empty_corner, 'X',
                 [['O', EMP, 'X'],
                  [EMP, EMP, EMP],
                  ['O', EMP, 'X']]);

board_test_some!(some_empty_corner_test, empty_corner, 'X',
                 [[EMP, 'O', EMP],
                  ['O', 'X', 'O'],
                  [EMP, 'X', EMP]],
                 Position {row: 0, col: 0});

// testing empty_side()
board_test_none!(none_empty_side_test, empty_side, 'X',
                 [[EMP, 'O', EMP],
                  ['O', 'X', 'O'],
                  [EMP, 'X', EMP]]);

board_test_some!(some_empty_side_test, empty_side, 'X',
                 [['O', EMP, 'X'],
                  [EMP, EMP, EMP],
                  ['O', EMP, 'X']],
                 Position {row: 1, col: 0});

#[test]
fn valid_move_count_test() {
    // winning move
    let mut board: Board = 
        [['X', EMP, EMP],
         ['O', 'O', EMP],
         ['X', EMP, EMP]];
    assert_eq!(valid_move_count('X', &board, win), 0);

    board =
        [['X', 'O', 'X'],
         ['O', EMP, 'O'],
         ['X', EMP, EMP]];
    assert_eq!(valid_move_count('X', &board, win), 1);

    board =
        [['X', EMP, 'X'],
         [EMP, 'O', EMP],
         ['X', EMP, 'O']];
    assert_eq!(valid_move_count('X', &board, win), 2);

    // forking move
    board =
        [['X', 'O', EMP],
         ['X', 'O', EMP],
         ['O', EMP, 'X']];
    assert_eq!(valid_move_count('X', &board, fork), 0);

    board =
        [['X', 'O', EMP],
         [EMP, EMP, 'X'],
         [EMP, EMP, 'O']];
    assert_eq!(valid_move_count('X', &board, fork), 1);

    board =
        [['X', EMP, EMP],
         [EMP, 'O', EMP],
         [EMP, EMP, 'X']];
    assert_eq!(valid_move_count('X', &board, fork), 2);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct GameState {
    last_player: char,
    board: Board,
}

fn brute_force_helper(current_player: char, bot_player: char, 
                      mut board: Board, failed_games: &mut HashSet<GameState>) {
    // base case: game is tied
    if is_tied(&board) {
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
        make_move(current_player, &mut board);
        brute_force_helper(opposite_player(current_player), bot_player, 
                           board, failed_games);
    // if it's the non-bot's turn, recursively call for every possible move
    } else {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == EMPTY_SQUARE {
                    let mut board_copy = board.clone();
                    board_copy[i][j] = current_player;
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
    brute_force_helper('X', 'X', [[EMPTY_SQUARE; 3]; 3], &mut failed_games);

    // test for bot == 'O'
    brute_force_helper('X', 'O', [[EMPTY_SQUARE; 3]; 3], &mut failed_games);
    assert!(failed_games.is_empty(), "Failed game states were:\n {:?}", failed_games);
}
