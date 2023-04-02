use crate::*;

pub fn alpha_beta_negamax(board: &mut Vec<Vec<i32>>, player: i32, depth: i32, mut alpha: i32, beta: i32) -> (i32, i32, i32) {
    let mut best_pos: (i32, i32) = (-1, -1);
    let mut best_score = std::i32::MIN + 2;
    
    if depth == 0  || game_over(board, player, 0) {
        // Evaluate the board using a heuristic function
        let score = evaluate_board(board, player);
        return (-1, -1, score);
    }
    
    for (x, y) in get_candidates(board, player) {
        println!("Placing x:{}, y:{} | depth: {}", x, y, depth);
        // Play the move
        board[x as usize][y as usize] = player;
        
        // Evaluate the move recursively
        let (_, _, score) = alpha_beta_negamax(
            board,
            if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE},
            depth - 1,
            -beta,
            -alpha
        );
        
        
        let score = -score;
        
        // Undo the move
        board[x as usize][y as usize] = UNPLAYED_STATE;
        
        // Check if this move is the best so far
        if score > best_score {
            println!("-> Changing best at {} depth | old_best: {}, new_score: {}", depth, best_score, score);
            best_score = score;
            best_pos = (x.clone(), y.clone());
        }
        
        // Apply alpha-beta pruning
        alpha = if best_score > alpha {best_score} else {alpha};
        if alpha >= beta {
            
            println!("Alpha Over Beta: Prunning | {} > {} | score: {} | depth : {}", alpha, beta, best_score, depth);
            return (best_pos.0, best_pos.1, best_score);
        }
    }
    // TODO: Protect from error return (only call if a piece is placable)
    return (best_pos.0, best_pos.1, best_score);
}
