use std::io::{self, Write};

use tic_tac_torture::*;

const DISPLAY_EMPTY_SQUARE: char = '*';

fn print_instructions() {
    println!("Welcome to Tic Tac Toe");
    println!("Use x,y coordinates to choose your position.");
    println!("Coordinates start with 1,1 in the top left and go to 3,3 in the bottom right");
    println!();
}

fn print_board(game: &Game) {
    println!("\nBoard state is:\n");
    for i in 0..game.board.len() {
        print!("  ");
        for j in 0..game.board[0].len() {
            print!("{} ", match game.board[i][j] {
                Some(p) => p.to_char(),
                None => DISPLAY_EMPTY_SQUARE,
            });
        }
        println!()
    }
    println!();
}

fn print_exit_message(game: &Game) {
    let exit_message = match game.status {
        GameStatus::Tie => "Tie game.".to_string(),
        GameStatus::Win(player) => format!("Player {} has won!", player.to_char()),
        _ => return,
    };
    println!("{}", exit_message);
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

fn main() {
    let mut game = Game::new();
    print_instructions();

    // main game loop, breaks on win or tie
    while let GameStatus::InProgress = game.status {
        println!("It is player {}'s turn", game.current_player.to_char());
        print_board(&game);
        
        let (row, col) = if game.current_player_is_human() {
            // get user's move input
            let input = get_move_input();
            let (row, col) = match input {
                Ok(point) => point,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if let Some(_) = game.board[row][col] {
                println!("That spot is taken.");
                continue;
            }

            (row, col)
        } else {
            bot::get_move(game.current_player, &mut game.board)
        };

        game.make_move(row, col);
    }

    print_board(&game);
    print_exit_message(&game);
}
