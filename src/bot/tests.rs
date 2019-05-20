use super::*;

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

board_test_some!(double_block_fork_test, block_fork, 'O',
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
