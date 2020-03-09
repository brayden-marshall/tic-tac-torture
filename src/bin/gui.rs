extern crate piston_window;

use piston_window::*;

use tic_tac_torture::*;

const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 640.0;

const BACKGROUND_COLOR: [f32; 4] = [51.0/255.0, 51.0/255.0, 51.0/255.0, 1.0];
const GRID_COLOR: [f32; 4] = [150.0/255.0, 150.0/255.0, 150.0/255.0, 1.0];

const X_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const O_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

fn draw(game: &Game, context: &Context, graphics: &mut G2d) {
    let viewport = match context.viewport {
        Some(v) => v,
        None => panic!("Context doesn't have a viewport??? idk what that even means."),
    };

    clear(BACKGROUND_COLOR, graphics);

    let num_rows = game.board.len() as i32;
    let [x, y, width, height] = viewport.rect;
    let cell_width: i32 = width / num_rows;
    let cell_height: i32 = height / num_rows;


    for i in 0..num_rows {
        // draw game pieces
        let y = (y + (i * cell_height)) as f64;
        for j in 0..num_rows {
            let x = (x + (j * cell_width)) as f64;
            let cell_rect = [x, y, cell_width as f64, cell_height as f64];

            let cell = game.board[i as usize][j as usize];
            match cell {
                'X' => draw_x(context, graphics, cell_rect),
                'O' => draw_o(context, graphics, cell_rect),
                _ => (),
            }
        }
    }

    draw_grid(game, context, graphics);
}

fn draw_x(context: &Context, graphics: &mut G2d, rect: types::Rectangle) {
    let [x, y, width, height] = rect;

    let center_x = x + (width/2.0);
    let center_y = y + (height/2.0);

    let short_side = if width < height { width } else { height };

    let pad = short_side * 0.1;
    let line_width = short_side / 8.0;
    let line_length = short_side - 2.0*pad;
    let rect = [0.0, 0.0, line_width, line_length];

    let transform1 = context.transform
        .trans(center_x, center_y)
        .rot_deg(45.0)
        .trans(-line_width/2.0, -line_length/2.0);

    let transform2 = context.transform
        .trans(center_x, center_y)
        .rot_deg(-45.0)
        .trans(-line_width/2.0, -line_length/2.0);

    rectangle(X_COLOR, rect, transform1, graphics);
    rectangle(X_COLOR, rect, transform2, graphics);
}

fn draw_o(context: &Context, graphics: &mut G2d, rect: types::Rectangle) {
    let pad = 0.2;
    let line_width = 15.0;

    let [x, y, width, height] = rect;
    let center_x = x + (width/2.0);
    let center_y = y + (height/2.0);

    let outer_radius = if width < height { width } else { height } / 2.0;
    let outer_radius = outer_radius - outer_radius*pad;

    let inner_radius = outer_radius - line_width;

    let outer_rect = ellipse::centered([center_x, center_y, outer_radius, outer_radius]);
    let inner_rect = ellipse::centered([center_x, center_y, inner_radius, inner_radius]);

    ellipse(O_COLOR, outer_rect, context.transform, graphics);
    ellipse(BACKGROUND_COLOR, inner_rect, context.transform, graphics);
}

fn draw_grid(game: &Game, context: &Context, graphics: &mut G2d) {
    let viewport = match context.viewport {
        Some(v) => v,
        None => panic!("Context doesn't have a viewport??? idk what that even means."),
    };

    let num_rows = game.board.len() as i32;
    let [x, y, width, height] = viewport.rect;
    let cell_width: i32 = width / num_rows;
    let cell_height: i32 = height / num_rows;
    let line_radius: f64 = 2.0;
    for i in 0..=num_rows {
        // draw vertical grid lines
        let x0 = (x + (i * cell_width)) as f64;
        let y0: f64 = 0.0;
        let x1 = x0;
        let y1 = height as f64;

        line_from_to(
            GRID_COLOR, line_radius,
            [x0, y0], [x1, y1],
            context.transform, graphics
        );

        // draw horizontal grid lines
        let x0: f64 = 0.0;
        let y0 = (y + (i * cell_height)) as f64;
        let x1 = width as f64;
        let y1 = y0;

        line_from_to(
            GRID_COLOR, line_radius,
            [x0, y0], [x1, y1],
            context.transform, graphics
        );
    }
}

fn handle_click(board: &mut Board, current_player: &Player, draw_size: [u32; 2], cursor_pos: [f64; 2]) -> bool{
    if !current_player.is_human {
        return false;
    }

    let num_rows = board.len();

    let cell_width  = draw_size[0] / num_rows as u32;
    let cell_height = draw_size[1] / num_rows as u32;

    let row: usize = (cursor_pos[1] as u64 / cell_height as u64) as usize;
    let col: usize = (cursor_pos[0] as u64 / cell_width as u64) as usize;

    if board[row][col] != EMPTY_SQUARE {
        // FIXME: do something to signal spot is taken
        println!("That spot is taken.");
        return false;
    }

    board[row][col] = current_player.token;
    true
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut game = Game::new();
    game.player2.is_human = true;
    //game.board[0][0] = 'X';
    //game.board[2][1] = 'X';
    //game.board[1][1] = 'O';
    //game.board[2][2] = 'O';

    let mut cursor_pos: [f64; 2] = [0.0, 0.0];
    let mut draw_size: [u32; 2] = [0, 0];
    let mut current_player = Some(&game.player1);
    while let Some(event) = window.next() {
        if let Some(render_args) = event.render_args() {
            draw_size = render_args.draw_size;
            window.draw_2d(&event, |context, graphics, _device| {
                draw(&game, &context, graphics);
            });
        }

        if let Some(pos) = event.mouse_cursor_args() {
            cursor_pos[0] = pos[0];
            cursor_pos[1] = pos[1];
        }

        if let Some(button_args) = event.button_args() {
            if let ButtonState::Press = button_args.state {
                if let GameStatus::Win(_) | GameStatus::Tie = game.status.clone() {
                    current_player = None;
                    game.reset();
                    current_player = Some(&game.player1);
                } else if let Button::Mouse(MouseButton::Left) = button_args.button {
                    let move_was_made = handle_click(&mut game.board, &current_player.unwrap(), draw_size, cursor_pos);

                    if move_was_made {
                        if has_won(current_player.unwrap().token, &game.board) {
                            game.status = GameStatus::Win(current_player.unwrap().token);
                            println!(
                                "Winner winner, chicken chinner. Player {} won.",
                                current_player.unwrap().token,
                            );
                        }

                        if is_full(&game.board) {
                            game.status = GameStatus::Tie;
                            println!("It's a tie.");
                        }

                        current_player = if current_player.unwrap() == &game.player1 {
                            Some(&game.player2)
                        } else {
                            Some(&game.player1)
                        };
                    }
                }
            }
        }
    }
}
