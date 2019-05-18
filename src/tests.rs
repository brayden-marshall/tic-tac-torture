use super::*;

// renaming in test module for clearer visuals in test cases
const EMP: char = EMPTY_SQUARE;

#[test]
fn horizontal_win_check_test() {
    let mut board: Board = 
        [['X', 'X', 'X'],
         [EMP, EMP, EMP],
         [EMP, EMP, EMP]];
    assert!(has_won('X', board));

    board = 
        [[EMP, EMP, EMP],
         ['X', 'X', 'X'],
         [EMP, EMP, EMP]];
    assert!(has_won('X', board));

    board = 
        [[EMP, EMP, EMP],
         [EMP, EMP, EMP],
         ['X', 'X', 'X']];
    assert!(has_won('X', board));
}

#[test]
fn vertical_win_check_test() {
    let mut board: Board = 
        [['O', EMP, EMP],
         ['O', EMP, EMP],
         ['O', EMP, EMP]];
    assert!(has_won('O', board));

    board = 
        [[EMP, 'O', EMP],
         [EMP, 'O', EMP],
         [EMP, 'O', EMP]];
    assert!(has_won('O', board));

    board = 
        [[EMP, EMP, 'O'],
         [EMP, EMP, 'O'],
         [EMP, EMP, 'O']];
    assert!(has_won('O', board));
}

#[test]
fn diagonal_win_check_test() {
    let mut board: Board =
        [['X', EMP, EMP],
         [EMP, 'X', EMP],
         [EMP, EMP, 'X']];
    assert!(has_won('X', board));

    board =
        [[EMP, EMP, 'X'],
         [EMP, 'X', EMP],
         ['X', EMP, EMP]];
    assert!(has_won('X', board));
}

/********************************
 ***** Bot Function Tests *******
 *******************************/

fn bot_win_test() {
}
