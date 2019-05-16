use super::*;

#[test]
fn horizontal_win_test() {
    let mut board: Board = 
        [[Some('X'), Some('X'), Some('X')],
         [None, None, None],
         [None, None, None]];
    assert!(has_won('X', board));

    board = 
        [[None, None, None],
         [Some('X'), Some('X'), Some('X')],
         [None, None, None]];
    assert!(has_won('X', board));

    board = 
        [[None, None, None],
         [None, None, None],
         [Some('X'), Some('X'), Some('X')]];
    assert!(has_won('X', board));
}

#[test]
fn vertical_win_test() {
    let mut board: Board = 
        [[Some('O'), None, None],
         [Some('O'), None, None],
         [Some('O'), None, None]];
    assert!(has_won('O', board));

    board = 
        [[None, Some('O'), None],
         [None, Some('O'), None],
         [None, Some('O'), None]];
    assert!(has_won('O', board));

    board = 
        [[None, None, Some('O')],
         [None, None, Some('O')],
         [None, None, Some('O')]];
    assert!(has_won('O', board));
}

#[test]
fn diagonal_win_test() {
    let mut board: Board =
        [[Some('X'), None, None],
         [None, Some('X'), None],
         [None, None, Some('X')]];
    assert!(has_won('X', board));

    board =
        [[None, None, Some('X')],
         [None, Some('X'), None],
         [Some('X'), None, None]];
    assert!(has_won('X', board));
}
