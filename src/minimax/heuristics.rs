use crate::*;

pub fn winner_board(board: &mut Vec<Vec<i32>>, player : i32) -> bool {
    
    // According to https://www.scirp.org/journal/paperinformation.aspx?paperid=90972
    // Feature 1: Absolute win
    //      Five chessmen are connected horizontally, vertically or diagonally as shown in Figure 5: (a, 1)-(b, 2)-(c, 3)-(d, 4).
    // Feature 2: Three connected chessmen
    //      Four chessmen are connected horizontally, vertically or diagonally as shown in Figure 5: (c, 2)-(d, 2)-(e, 2) or (c, 1)-(d, 2)-(f, 4).
    // Feature 3: Two connected chessmen
    //      Three chessmen are connected horizontally, vertically or diagonally as shown in Figure 5: (b, 1)-(c, 1).
    // Feature 4: Single chessman
    //      A chessman that is not connected to another same chessman horizontally, vertically or diagonally as shown in Figure 5: (d, 1).
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            // Heuristic for candidate selection
            if is_candidate(board, x as i32, y as i32, player) {
                if is_winner(board, x as i32, y as i32, player) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn evaluate_board(board: &mut Vec<Vec<i32>>, player : i32) -> i32 {
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            // Heuristic for candidate selection
            if is_candidate(board, x as i32, y as i32, player) {
                if is_winner(board, x as i32, y as i32, player) {
                    return std::i32::MAX - 5;
                }
            }
        }
    }
    1
}

pub fn is_candidate(board: &Vec<Vec<i32>>, x: i32, y: i32, player: i32) -> bool {
    size = board.len() as i32;
    if is_legal(board, x, y, player) {
        for n in get_neighbours(x, y, board[0].len() as i32) { 
            if board[n.0 as usize][n.1 as usize] != UNPLAYED_STATE {
                return true;
            }
        }
    }
    false
}