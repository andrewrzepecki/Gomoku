use crate::*;

pub fn alpha_beta_negamax(
    board: &mut Board,
    player: i32,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    tt: &mut HashMap<String, (i32, i32, i32)>,
) -> (i32, i32, i32) {
    
    // Transposition Table
    let board_hash = board.get_hash();
    if let Some(entry) = tt.get(&board_hash) {
        return (entry.0, entry.1, entry.2)
    }
    
    // Return Score
    if depth == 0 || board.game_over(player) {
        let score = evaluate_board(board, player);//- evaluate_board(board, board.get_opponent(player));
        return (-1, -1, score);
    }

    let mut best_pos: (i32, i32) = (-1, -1);
    let mut best_score = std::i32::MIN + 2;

    // Get moves in order. 
    for mut m in get_moves(board, player) {
        
        // Play the move
        m.set(board);
        
        // Evaluate the move recursively
        let (_, _, score) = alpha_beta_negamax(
            board,
            board.get_opponent(player),
            depth - 1,
            -beta,
            -alpha,
            tt,
        );
        
        // Undo the move
        m.unset(board);
        
        // Check if this move is the best so far
        let score = -score;
        if score > best_score {
            best_score = score;
            best_pos = (m.x.clone(), m.y.clone());
        }
        
        // Apply alpha-beta pruning
        alpha = if best_score > alpha {best_score} else {alpha};
        if alpha >= beta {
            break;
        }
    }
    tt.insert(board_hash, (best_pos.0, best_pos.1, best_score));
    return (best_pos.0, best_pos.1, best_score);
}
