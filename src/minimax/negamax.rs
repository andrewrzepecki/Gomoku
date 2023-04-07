use crate::*;


pub fn alpha_beta_negamax(
    board: &mut Board,
    player: i32,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    tt: &mut HashMap<String, (i32, i32, i32, i32, i32)>,
) -> (i32, i32, i32) {

    let board_hash = board.get_hash(player);
    if let Some(&(x, y, s, a, b)) = tt.get(&board_hash) {
        let best_move = BoardMove::new(x, y, player);
        if board.is_legal_move(best_move.x, best_move.y, player) {
            if a < beta || b > alpha {
                return (x, y, s);
            }
        }
    }

    // Return Score
    if depth == 0 || board.game_over(player) {
        let score = get_final_score(board, player);
        return (-1, -1, score);
    }

    let mut best_move = (-1, -1);
    let mut best_score = std::i32::MIN + 2;

    // Get moves in order.
    let moves = get_moves(board, player);
    for mut m in moves {
        
        // Play the move
        m.set(board);
        if board.game_over(player) {
            m.unset(board);
            return (m.x, m.y, 1000000);
        }
        if board.has_live_four(board.get_opponent(player)) {
            m.unset(board);  
            continue;
        }
        
        // Evaluate the move recursively
        let (_, _, score) = alpha_beta_negamax(
            &mut board.clone(),
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
            best_move = (m.x, m.y);
        }
        
        // Apply alpha-beta pruning
        alpha = if best_score > alpha {best_score} else {alpha};
        if alpha >= beta {
            break;
        }
    }

    // Insert Entry into
    if  best_move.0 != -1 {
        tt.insert(board.get_hash(player), (best_move.0, best_move.1, best_score, alpha, beta));
    }
    return (best_move.0, best_move.1, best_score);
}
