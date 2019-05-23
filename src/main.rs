use std::io::{self, Write};
use std::fmt;

#[cfg(test)]
mod tests;
mod bot;

const EMPTY_SQUARE: char = '*';

#[derive(Debug, PartialEq)]
struct Player {
    token: char,
    is_human: bool,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

type Board = [[char; 3]; 3];

enum GameStatus {
    InProgress,
    Tie,
    Win(char),
}

struct Game {
    player1: Player,
    player2: Player,
    board: Board,
    status: GameStatus,
}

impl Game {
    fn new() -> Game {
        Game {
            player1: Player {
                token: 'X',
                is_human: true,
            },
            player2: Player {
                token: 'O',
                is_human: false,
            },
            board: [[EMPTY_SQUARE; 3]; 3],
            status: GameStatus::InProgress,
        }
    }

    fn print_instructions() {
        println!("Welcome to Tic Tac Toe");
        println!("FIXME: write some instructions");
    }

    fn print_board(&self) {
        println!("\nBoard state is:\n");
        for i in 0..self.board.len() {
            print!("  ");
            for j in 0..self.board[0].len() {
                print!("{} ", self.board[i][j]);
            }
            println!()
        }
        println!();
    }

    fn print_exit_message(&self) {
        let exit_message = match self.status {
            GameStatus::Tie => "Tie game.".to_string(),
            GameStatus::Win(t) => format!("Player {} has won!", t),
            _ => return,
        };
        println!("{}", exit_message);
    }

    fn play(&mut self) {
        Game::print_instructions();

        let mut current_player = &self.player1;
        // main game loop, breaks on win or tie
        while let GameStatus::InProgress = self.status {
            println!("It is player {}'s turn", current_player);
            self.print_board();
            
            if current_player.is_human {
                let (x, y) = get_move_input();

                if self.board[x][y] != EMPTY_SQUARE {
                    println!("That spot is taken.");
                    continue;
                }
                self.board[x][y] = current_player.token;
            } else {
                bot::make_move(current_player.token, &mut self.board);
            }

            if has_won(current_player.token, self.board) {
                self.status = GameStatus::Win(current_player.token);
                break;
            }

            if is_tied(&self.board) {
                self.status = GameStatus::Tie;
                break;
            }

            // switch to the next player's turn
            current_player = if current_player == &self.player1 {
                &self.player2
            } else {
                &self.player1
            }
        }

        self.print_board();
        self.print_exit_message();
    }
}

fn check_direction(player: char, board: &Board, row: usize, col: usize, offset_row: i32, offset_col: i32) -> bool {
    // assert that x and y are in bounds
    assert!(row < board.len());
    assert!(col < board.len());

    let mut i = row as i32;
    let mut j = col as i32;
    for _ in 0..board.len() {
        // if board location is anything but the player, return false
        if board[i as usize][j as usize] != player {
            return false;
        }

        i += offset_row;     
        j += offset_col;
    }
    return true;
}

fn has_won(player: char, board: Board) -> bool {
    for i in 0..board.len() {
        // check horizontal
        if check_direction(player, &board, i, 0, 0, 1) {
            return true;
        }

        // check vertical
        if check_direction(player, &board, 0, i, 1, 0) {
            return true;
        }
    }

    // check diagonals
    if check_direction(player, &board, 0, 0, 1, 1) {
        return true;
    }

    if check_direction(player, &board, 0, 2, 1, -1) {
        return true;
    }

    return false;
}

fn is_tied(board: &Board) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == EMPTY_SQUARE {
                return false;
            }
        }
    }
    true
}

// expects input from range [1, 3], returns as range [0, 2]
fn get_move_input() -> (usize, usize) {
    println!("Enter the position in which you would like to play.");
    let mut x = String::new();
    let mut y = String::new();
    
    print!("x >> ");
    // we have to flush stdout here to force it to print
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut x)
        .expect("Failed to read user input.");

    print!("y >> ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut y)
        .expect("Failed to read user input.");

    println!();

    let x: usize = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input must be an integer from 1-3");
            return get_move_input()
        }
    };

    let y: usize = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input must be an integer from 1-3");
            return get_move_input()
        }
    };

    // check if x and y are in range
    if x >= 1 && x <= 3 && y >= 1 && y <= 3 {
        // subtracting 1 to start counting from zero
        (x-1, y-1)
    } else {
        println!("coordinates must be in range [0,2]");
        get_move_input()
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
