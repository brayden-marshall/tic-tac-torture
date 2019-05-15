use std::io::{self, Write};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    X,
    O,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self { Token::X => 'X', Token::O => 'O' })
    }
}

type Board = [[Option<Token>; 3]; 3];

struct Game {
    player1: Token,
    player2: Token,
    board: Board,
}

impl Game {
    fn new() -> Game {
        Game {
            player1: Token::X,
            player2: Token::O,
            board: [[None; 3]; 3]
        }
    }

    fn print_instructions() {
        println!("Welcome to Tic Tac Toe");
    }

    fn print_board(&self) {
        println!("\nBoard state is:\n");
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if let Some(t) = self.board[i][j] {
                    print!("{} ", t);
                } else {
                    print!("* ");
                }
            }
            println!()
        }
    }

    fn play(&mut self) {
        Game::print_instructions();

        let mut current_player = self.player1;
        loop {
            println!("\nIt is player {}'s turn", current_player);
            self.print_board();

            let (x, y) = get_move_input();

            match self.board[x][y] {
                Some(_) => {
                    println!("\nThat spot is taken.");
                    continue
                },
                None => self.board[x][y] = Some(current_player),
            }

            if has_won(current_player, self.board) {
                self.print_board();
                println!("Winner is {}", current_player);
                break;
            }

            if is_tied(self.board) {
                println!("Tie game.");
                break;
            }

            // switch to the next player's turn
            current_player = if let Token::X = current_player {
                self.player2
            } else {
                self.player1
            }
        }
    }
}

fn check_direction(player: Token, board: Board, x: usize, y: usize, offset_x: i32, offset_y: i32) -> bool {
    // assert that x and y are in bounds
    assert!(x < board.len());
    assert!(y < board.len());

    let mut i = x as i32;
    let mut j = y as i32;
    for _ in 0..board.len() {
        // if board location is anything but the player, return false
        match board[i as usize][j as usize] {
            Some(t) if t == player => (),
            _ => return false,
        }

        i += offset_x;     
        j += offset_y;
    }
    return true;
}

fn has_won(player: Token, board: Board) -> bool {
    // check horizontal
    for i in 0..board.len() {
        if check_direction(player, board, i, 0, 0, 1) {
            return true;
        }
    }

    // check vertical
    for i in 0..board.len() {
        if check_direction(player, board, 0, i, 1, 0) {
            return true;
        }
    }

    // check diagonals
    if check_direction(player, board, 0, 0, 1, 1) {
        return true;
    }
    if check_direction(player, board, 0, 2, 1, -1) {
        return true;
    }
    return false;
}

fn is_tied(board: Board) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if let None = board[i][j] {
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

    let x: usize = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!()
    };

    let y: usize = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!()
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

#[cfg(test)]
mod tests;
