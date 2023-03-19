
use crate::{UNPLAYED_STATE, PLAYER1_STATE, PLAYER2_STATE,
            utils::{get_neighbours, is_valid_coords}, game_data::AppState};


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

fn is_double_three(board : &Vec<Vec<i32>>, x : i32, y : i32, player : i32) -> bool {
    let size = board[0].len() as i32;
    let mut doubles = 0;
    let opp_player = if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
    let mut copy_board = board.clone();
    copy_board[x as usize][y as usize] = player;
    for i in 0..size {
        for j in 0..size {
            if copy_board[i as usize][j as usize] == player {
                let mut s_count = 1;
                let mut b_count = 0;
                for n in get_neighbours(x, y, size) {
                    if copy_board[n.0 as usize][n.1 as usize] == player || copy_board[n.0 as usize][n.1 as usize] == UNPLAYED_STATE {
                        let x2 = x + ((n.0 - x) * 2);
                        let y2 = y + ((n.1 - y) * 2);
                        let x3 = x + ((n.0 - x) * 3);
                        let y3 = y + ((n.1 - y) * 3);
                        if is_valid_coords(x2, y2, size) && is_valid_coords(x3, y3, size) {
                            let p2 = copy_board[x2 as usize][y2 as usize];
                            let p3 = copy_board[x3 as usize][y3 as usize];
                            if copy_board[n.0 as usize][n.1 as usize] == UNPLAYED_STATE {
                                b_count += 1;
                            }
                            else {
                                s_count += 1;
                            }
                            if p2 == UNPLAYED_STATE {
                                b_count += 1;
                            }
                            else if p2 == player {
                                s_count += 1;
                            }
                            if p3 == UNPLAYED_STATE {
                                b_count += 1;
                            }
                            else if p3 == player {
                                s_count += 1;
                            }
                            if s_count == 3 && b_count == 1 {
                                doubles += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    if doubles > 1 {
        return true;
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
    if is_double_three(board, x, y, player) {
        return false;
    }
    return true;
}

pub fn check_capture(board : &mut Vec<Vec<i32>>, x : i32, y : i32, player : i32) -> bool {
    let size = board[0].len() as i32;
    let opp_player = if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
    for n in get_neighbours(x, y, size) {
        if board[n.0 as usize][n.1 as usize] == opp_player {
            let x_sym = x + ((n.0 - x) * 2);
            let y_sym = y + ((n.1 - y) * 2);
            let x_opp = x + ((n.0 - x) * 3);
            let y_opp = y + ((n.1 - y) * 3);
            if is_valid_coords(x_sym, y_sym, size) && is_valid_coords(x_opp, y_opp, size) {
                if board[x_sym as usize][y_sym as usize] == opp_player && board[x_opp as usize][y_opp as usize] == player {
                    board[x_sym as usize][y_sym as usize] = UNPLAYED_STATE;
                    board[n.0 as usize][n.1 as usize] = UNPLAYED_STATE;
                    return true;
                }
            }
        }
    }
    return false;
}