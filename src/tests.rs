use super::*;

// renaming in test module for clearer visuals in test cases
static EMP: Option<PlayerKind> = None;
static P_X: Option<PlayerKind> = Some(PlayerKind::PlayerX);
static P_O: Option<PlayerKind> = Some(PlayerKind::PlayerO);

use PlayerKind::*;

#[test]
fn horizontal_win_test() {
    let mut board: Board = 
        [[P_X, P_X, P_X],
         [EMP, EMP, EMP],
         [EMP, EMP, EMP]];
    assert!(has_won(PlayerX, &board));

    board = 
        [[EMP, EMP, EMP],
         [P_X, P_X, P_X],
         [EMP, EMP, EMP]];
    assert!(has_won(PlayerX, &board));

    board = 
        [[EMP, EMP, EMP],
         [EMP, EMP, EMP],
         [P_X, P_X, P_X]];
    assert!(has_won(PlayerX, &board));
}

#[test]
fn vertical_win_test() {
    let mut board: Board = 
        [[P_O, EMP, EMP],
         [P_O, EMP, EMP],
         [P_O, EMP, EMP]];
    assert!(has_won(PlayerO, &board));

    board = 
        [[EMP, P_O, EMP],
         [EMP, P_O, EMP],
         [EMP, P_O, EMP]];
    assert!(has_won(PlayerO, &board));

    board = 
        [[EMP, EMP, P_O],
         [EMP, EMP, P_O],
         [EMP, EMP, P_O]];
    assert!(has_won(PlayerO, &board));
}

#[test]
fn diagonal_win_test() {
    let mut board: Board =
        [[P_X, EMP, EMP],
         [EMP, P_X, EMP],
         [EMP, EMP, P_X]];
    assert!(has_won(PlayerX, &board));

    board =
        [[EMP, EMP, P_X],
         [EMP, P_X, EMP],
         [P_X, EMP, EMP]];
    assert!(has_won(PlayerX, &board));
}

#[test]
fn tie_test() {
    let board: Board =
        [[P_X, P_O, P_X],
         [P_O, P_X, P_X],
         [P_O, P_X, P_O]];
    assert!(is_full(&board));
}

#[test]
fn no_tie_test() {
    let board: Board =
        [[P_X, P_O, P_X],
         [P_O, EMP, P_O],
         [P_O, P_X, P_X]];
    assert!(!is_full(&board));
}
