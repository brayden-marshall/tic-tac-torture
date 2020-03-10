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
    Win(PlayerKind),
}

pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: PlayerKind,
    pub board: Board,
    pub status: GameStatus,
}

impl Game {
    pub fn new() -> Game {
        use PlayerKind::*;
        Game {
            player1: Player {
                kind: PlayerX,
                is_human: true,
            },
            player2: Player {
                kind: PlayerO,
                is_human: false,
            },
            current_player: PlayerX,
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            status: GameStatus::InProgress,
        }
    }

    pub fn reset(&mut self) {
        self.board = [[None; BOARD_SIZE]; BOARD_SIZE];
        self.status = GameStatus::InProgress;
        self.current_player = PlayerKind::PlayerX;
    }

    // @Hack this doesn't smell very good
    pub fn current_player_is_human(&self) -> bool {
        if self.player1.kind == self.current_player {
            return self.player1.is_human;
        } 

        if self.player2.kind == self.current_player {
            return self.player2.is_human;
        }

        return false;
    }

    pub fn make_move(&mut self, row: usize, col: usize) {
        use PlayerKind::*;

        self.board[row][col] = Some(self.current_player);

        if has_won(self.current_player, &self.board) {
            self.status = GameStatus::Win(self.current_player);
        }

        if is_full(&self.board) {
            self.status = GameStatus::Tie;
        }

        self.current_player = match self.current_player {
            PlayerX => PlayerO,
            PlayerO => PlayerX,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerKind {
    PlayerX,
    PlayerO,
}

impl PlayerKind {
    pub fn to_char(&self) -> char {
        use PlayerKind::*;
        match self {
            PlayerX => 'X',
            PlayerO => 'O',
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Player {
    pub kind: PlayerKind,
    pub is_human: bool,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind.to_char())
    }
}

pub type Board = [[Option<PlayerKind>; BOARD_SIZE]; BOARD_SIZE];

pub fn has_won(player: PlayerKind, board: &Board) -> bool {
    let n = board.len();

    let mut diagonal_1_count = 0;
    let mut diagonal_2_count = 0;
    for i in 0..n {
        let mut row_count = 0;
        let mut col_count = 0;
        for j in 0..n {
            // check row
            if let Some(cell) = board[i][j] {
                if cell == player {
                    row_count += 1;
                }
            }

            // check column
            if let Some(cell) = board[j][i] {
                if cell == player {
                    col_count += 1;
                }
            }
        }

        if row_count == n || col_count == n {
            return true;
        }

        // check NW to SE diagonal
        if let Some(cell) = board[i][i] {
            if cell == player {
                diagonal_1_count += 1;
            }
        }

        // check NE to SW diagonal
        if let Some(cell) = board[i][n-i-1] {
            if cell == player {
                diagonal_2_count += 1;
            }
        }
    }

    return diagonal_1_count == n || diagonal_2_count == n;
}

pub fn is_full(board: &Board) -> bool {
    for row in board.iter() {
        for square in row.iter() {
            if let None = *square {
                return false;
            }
        }
    }
    true
}
