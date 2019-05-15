use super::*;

#[test]
fn horizontal_win_test() {
    let mut board: Board = 
        [[Some(Token::X), Some(Token::X), Some(Token::X)],
         [None, None, None],
         [None, None, None]];
    assert!(has_won(Token::X, board));

    board = 
        [[None, None, None],
         [Some(Token::X), Some(Token::X), Some(Token::X)],
         [None, None, None]];
    assert!(has_won(Token::X, board));

    board = 
        [[None, None, None],
         [None, None, None],
         [Some(Token::X), Some(Token::X), Some(Token::X)]];
    assert!(has_won(Token::X, board));
}

#[test]
fn vertical_win_test() {
    let mut board: Board = 
        [[Some(Token::O), None, None],
         [Some(Token::O), None, None],
         [Some(Token::O), None, None]];
    assert!(has_won(Token::O, board));

    board = 
        [[None, Some(Token::O), None],
         [None, Some(Token::O), None],
         [None, Some(Token::O), None]];
    assert!(has_won(Token::O, board));

    board = 
        [[None, None, Some(Token::O)],
         [None, None, Some(Token::O)],
         [None, None, Some(Token::O)]];
    assert!(has_won(Token::O, board));
}

#[test]
fn diagonal_win_test() {
    let mut board: Board =
        [[Some(Token::X), None, None],
         [None, Some(Token::X), None],
         [None, None, Some(Token::X)]];
    assert!(has_won(Token::X, board));

    board =
        [[None, None, Some(Token::X)],
         [None, Some(Token::X), None],
         [Some(Token::X), None, None]];
    assert!(has_won(Token::X, board));
}
