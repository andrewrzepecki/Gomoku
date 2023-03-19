
use crate::{UNPLAYED_STATE, PLAYER1_STATE, PLAYER2_STATE,
            utils::{get_neighbours, is_valid_coords}};


fn is_illegal_capture(board : &Vec<Vec<i32>>, x :i32, y: i32, player : i32) -> bool {
    let size = board[0].len() as i32;
    let opp_player = if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
    for n in get_neighbours(x, y, size) {
        if board[n.0 as usize][n.1 as usize] == player {
            let x_sym = x + ((n.0 - x) * 2);
            let y_sym = y + ((n.1 - y) * 2);
            let x_opp = x + ((n.0 - x) * -1);
            let y_opp = y + ((n.1 - y) * -1);
            if is_valid_coords(x_sym, y_sym, size) && is_valid_coords(x_opp, y_opp, size) {
                if board[x_sym as usize][y_sym as usize] == opp_player && board[x_opp as usize][y_opp as usize] == opp_player {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn is_legal(board : &Vec<Vec<i32>>, x : i32, y : i32, player : i32) -> bool {
    if UNPLAYED_STATE != board[x as usize][y as usize] {
        return false;
    }
    if is_illegal_capture(board, x, y, player) {
        return false;
    }
    return true;
}