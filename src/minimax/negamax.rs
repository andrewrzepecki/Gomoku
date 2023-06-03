use crate::*;

pub fn alpha_beta_negamax(
    board: &mut Board,
    player: Players,
    depth: usize,
    mut alpha: i32,
    beta: i32,
) -> (usize, usize, i32) {

    // Return Score
    if depth == 0 || board.winner != Players::Unplayed {
        let score = get_board_score(board, player);
        return (42, 42, score);
    }

    let mut best = (42 as usize, 42 as usize, i32::MIN);

    // Get moves in order.
    for (x, y, _) in get_candidate_moves(board, player) {

        // Play the move
        if x == y && x == 42 { continue }
        let mut m = BoardMove::new(x, y, player);
        m.set(board);
        if m.is_winner {
            m.unset(board);
            return (m.x, m.y, 200000 as i32);
        }

        // Evaluate the move recursively
        let (_, _, score) = alpha_beta_negamax(
            board,
            get_opponent(player),
            depth - 1,
            -beta,
            -alpha,
        );

        // Undo the move
        m.unset(board);

        // Check if this move is the best so far
        let score = -score;
        if score > best.2 {
            best = (m.x, m.y, score);
        }

        // Apply alpha-beta pruning
        alpha = if best.2 > alpha {best.2} else {alpha};
        if alpha >= beta {
            break;
        }
    }
    return best;
}
