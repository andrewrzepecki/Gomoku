/////////////////////////////////////////
/// ////////////////////////////////////
/// ///////////////////////////////////
use crate::*;




pub fn alpha_beta_minimax(
    board: &mut Board,
    player: i32,
    depth: i32,
    maximizing : bool,
    mut alpha: i32,
    mut beta: i32,
    tt: &mut HashMap<String, (i32, i32, i32)>,
) -> (i32, i32, i32) {

     // Transposition Table
    //let board_hash = board.get_hash() + if maximizing {"M"} else {"m"};
    //if let Some(entry) = tt.get(&board_hash) {
    //    return (entry.0, entry.1, entry.2)
    //}
    // Return Score
    if depth == 0 || board.game_over(player) {
        let mut score = get_final_score(board, player);
        //if !maximizing {
        //    score = -score;
        //}
        return (-1, -1, score);
    }

    let mut best_pos: (i32, i32) = (-1, -1);
    
    // Get moves in order. 
    if maximizing {
        let mut best_score = std::i32::MIN + 2;
        for mut m in get_moves(board, player) {
            
            // Play the move
            m.set(board);
        
            // Evaluate the move recursively
            let (_, _, score) = alpha_beta_minimax(board, board.get_opponent(player), depth - 1, false, alpha, beta, tt);
        
            // Undo the move
            m.unset(board);
            if score >= best_score {
                best_score = score;
                best_pos = (m.x.clone(), m.y.clone());
            }
            alpha = alpha.max(best_score);
            if beta <= alpha {
                break;
            }
        }
        //tt.insert(board.get_hash() + "M", (best_pos.0, best_pos.1, best_score));
        return (best_pos.0, best_pos.1, best_score);
    }
    else {
        let mut best_score = std::i32::MAX - 2;
        for mut m in get_moves(board, player) {
            // Play the move
            m.set(board);
        
            // Evaluate the move recursively
            let (_, _, score) = alpha_beta_minimax(board, board.get_opponent(player), depth - 1, true, alpha, beta, tt);
        
            // Undo the move
            m.unset(board);
            if score <= best_score {
                best_score = score;
                best_pos = (m.x.clone(), m.y.clone());
            }
            beta = beta.min(best_score);
            if beta <= alpha {
                break;
            }
        }
        //tt.insert(board.get_hash() + "m", (best_pos.0, best_pos.1, best_score));
        return (best_pos.0, best_pos.1, best_score);
    }

}