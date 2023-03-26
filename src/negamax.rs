use crate::*;

pub fn alpha_beta_negamax(board: &mut Vec<Vec<i32>>, player: i32, depth: i32, mut alpha: i32, beta: i32) -> (i32, i32, i32) {
    let mut best_pos: Option<(i32, i32)> = None;
    let mut best_score = std::i32::MIN;
    
    if depth == 0 {
        // Evaluate the board using a heuristic function
        let score = evaluate_board(board, player);
        return (-1, -1, score);
    }
    
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if is_legal(board, x as i32, y as i32, player) {
                // Play the move
                board[x][y] = player;
                
                // Evaluate the move recursively
                let (_, _, score) = alpha_beta_negamax(board, if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE}, depth - 1, -beta, -alpha);
                let score = -score;
                
                // Undo the move
                board[x][y] = 0;
                
                // Check if this move is the best so far
                if score > best_score {
                    best_score = score;
                    best_pos = Some((x as i32, y as i32));
                }
                
                // Apply alpha-beta pruning
                if best_score > alpha {
                    alpha = best_score;
                }
                
                if alpha >= beta {
                    return (best_pos.unwrap().0, best_pos.unwrap().1, best_score);
                }
            }
        }
    }
    return (best_pos.unwrap().0, best_pos.unwrap().1, best_score);
}

fn evaluate_board(board: &mut Vec<Vec<i32>>, player : i32) -> i32 {
    1
}