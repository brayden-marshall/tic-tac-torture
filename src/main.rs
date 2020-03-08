use std::io::{self, Write};

#[cfg(test)]
mod tests;
mod bot;
mod helpers;

use helpers::*;

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
            board: [[EMPTY_SQUARE; BOARD_SIZE]; BOARD_SIZE],
            status: GameStatus::InProgress,
        }
    }

    fn print_instructions() {
        println!("Welcome to Tic Tac Toe");
        println!("Use x,y coordinates to choose your position.");
        println!("Coordinates start with 1,1 in the top left and go to 3,3 in the bottom right");
        println!();
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
                // get user's move input
                let input = Game::get_move_input();
                let (x, y) = match input {
                    Ok(point) => point,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                if self.board[x][y] != EMPTY_SQUARE {
                    println!("That spot is taken.");
                    continue;
                }

                self.board[x][y] = current_player.token;
            } else {
                bot::make_move(current_player.token, &mut self.board);
            }

            if has_won(current_player.token, &self.board) {
                self.status = GameStatus::Win(current_player.token);
                break;
            }

            if is_full(&self.board) {
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
    // expects input from range [1, 3], returns as range [0, 2]
    fn get_move_input() -> Result<(usize, usize), &'static str> {
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
            Err(_) => return Err("Input must be an integer from 1-3"),
        };

        let y: usize = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Input must be an integer from 1-3"),
        };

        // check if x and y are in range
        if x >= 1 && x <= 3 && y >= 1 && y <= 3 {
            // subtracting 1 to start counting from zero
            Ok((x-1, y-1))
        } else {
            Err("Coordinates must be in range [1,3]")
        }
    }
}


fn main() {
    let mut game = Game::new();
    game.play();
}
