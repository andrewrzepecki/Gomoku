use crate::*;

pub fn alpha_beta_negamax(
    board: &mut Board,
    player: Players,
    depth: usize,
    mut alpha: i32,
    beta: i32,
) -> (usize, usize, i32) {

    // Return Score
    // if depth == 0 || board.has_winner {
    //     let score = board.get_score(player);
    //     return (-1, -1, score);
    // }

    // let mut best_move = (-1, -1);
    // let mut best_score = std::i32::MIN + 2;

    // Get moves in order.
    let moves = get_candidate_moves(board, player);
    for mut m in moves {
        return (m.0, m.1, m.2);
    }
    return (0, 0, 100);
    //     // Play the move
    //     m.set(board);
    //
    //     // Evaluate the move recursively
    //     let (_, _, score) = alpha_beta_negamax(
    //         &mut board.clone(),
    //         board.get_opponent(player),
    //         depth - 1,
    //         -beta,
    //         -alpha,
    //     );
    //
    //     // Undo the move
    //     m.unset(board);
    //
    //     // Check if this move is the best so far
    //     let score = -score;
    //     if score > best_score {
    //         best_score = score;
    //         best_move = (m.x, m.y);
    //     }
    //
    //     // Apply alpha-beta pruning
    //     alpha = if best_score > alpha {best_score} else {alpha};
    //     if alpha >= beta {
    //         break;
    //     }
    // }
    // return (best_move.0, best_move.1, best_score);
}
