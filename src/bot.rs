use super::Board;

pub fn make_move(player: char, board: &mut Board) {
    for i in 0..board.len() {
        for j in 0..board.len() {
            if let None = board[i][j] {
                board[i][j] = Some(player);
                return;
            }
        }
    }
}
