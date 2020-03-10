use std::fmt;

#[cfg(test)]
mod tests;
pub mod bot;

pub const EMPTY_SQUARE: char = '*';
pub const BOARD_SIZE: usize = 3;

#[derive(Clone)]
pub enum GameStatus {
    InProgress,
    Tie,
    Win(char),
}

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub board: Board,
    pub status: GameStatus,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Player {
                kind: PlayerKind::PlayerX,
                is_human: true,
            },
            player2: Player {
                kind: PlayerKind::PlayerO,
                is_human: false,
            },
            board: [[EMPTY_SQUARE; BOARD_SIZE]; BOARD_SIZE],
            status: GameStatus::InProgress,
        }
    }

    pub fn reset(&mut self) {
        self.board = [[EMPTY_SQUARE; BOARD_SIZE]; BOARD_SIZE];
        self.status = GameStatus::InProgress;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerKind {
    PlayerX,
    PlayerO,
}

impl PlayerKind {
    pub fn to_char(&self) -> char {
        match self {
            PlayerX => 'X',
            PlayerO => 'O',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Player {
    pub kind: PlayerKind,
    pub is_human: bool,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind.to_char())
    }
}

pub type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

pub fn has_won(player: char, board: &Board) -> bool {
    let n = board.len();

    let mut diagonal_1_count = 0;
    let mut diagonal_2_count = 0;
    for i in 0..n {
        let mut row_count = 0;
        let mut col_count = 0;
        for j in 0..n {
            // check row
            if board[i][j] == player {
                row_count += 1;
            }

            // check column
            if board[j][i] == player {
                col_count += 1;
            }
        }

        if row_count == n || col_count == n {
            return true;
        }

        // check NW to SE diagonal
        if board[i][i] == player {
            diagonal_1_count += 1;
        }

        // check NE to SW diagonal
        if board[i][n-i-1] == player {
            diagonal_2_count += 1;
        }
    }

    return diagonal_1_count == n || diagonal_2_count == n;
}

pub fn is_full(board: &Board) -> bool {
    for row in board.iter() {
        for square in row.iter() {
            if *square == EMPTY_SQUARE {
                return false;
            }
        }
    }
    true
}
