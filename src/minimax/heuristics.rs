use crate::*;

pub fn evaluate_board(board: &mut Vec<Vec<i32>>, player : i32) -> i32 {
    let mut score: i32 = 0;
    
    if is_winner_board(board) {
        return std::i32::MAX - 5;
    }
    score += 1;
    return score
}

pub fn is_candidate(board: &Vec<Vec<i32>>, x: i32, y: i32, player: i32) -> bool {
    let size = board[0].len() as i32;
    if is_legal(board, x, y, player) {
        for n in get_neighbours(x, y, size) { 
            if board[n.0 as usize][n.1 as usize] != UNPLAYED_STATE {
                return true;
            }
        }
    }
    false
}