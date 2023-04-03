use crate::*;

pub fn alpha_beta_negamax(
    board: &mut Board,
    player: i32,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    transposition_table: &mut HashMap<String, (i32, i32, i32)>,
) -> (i32, i32, i32) {
    let mut best_pos: (i32, i32) = (-1, -1);
    let mut best_score = std::i32::MIN + 2;
    
    // Transposition Table
    let board_hash = board.get_hash();
    if let Some(entry) = transposition_table.get(&board_hash) {
        return (entry.0, entry.1, entry.2)
    }
    // Return Score
    if depth == 0 || board.game_over(player) {
        // Evaluate the board using a heuristic function
        let score = evaluate_board(board, player);
        return (-1, -1, score);
    }


    // Get moves in order. 
    for mut m in get_moves(board, player) {
        //println!("Placing x:{}, y:{} | depth: {}", m.x, m.y, depth);
        
        // Play the move
        m.set(board);
        
        // Evaluate the move recursively
        let (_, _, score) = alpha_beta_negamax(
            board,
            board.get_opponent(player),
            depth - 1,
            -beta,
            -alpha,
            transposition_table,
        );
        
        // Undo the move
        m.unset(board);
        
        // Check if this move is the best so far
        let score = -score;
        if score > best_score {
            //println!("-> Changing best at {} depth | old_best: {}, new_score: {}", depth, best_score, score);
            best_score = score;
            best_pos = (m.x.clone(), m.y.clone());
        }
        
        // Apply alpha-beta pruning
        alpha = if best_score > alpha {best_score} else {alpha};
        if alpha >= beta {
            //println!("Alpha Over Beta: Prunning | {} > {} | score: {} | depth : {}", alpha, beta, best_score, depth);
            break;
            //return (best_pos.0, best_pos.1, best_score);
        }
    }
    // TODO: Protect from error return (only call if a piece is placable)
    transposition_table.insert(board_hash, (best_pos.0, best_pos.1, best_score));
    return (best_pos.0, best_pos.1, best_score);
}
