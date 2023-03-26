use crate::*;

pub fn alpha_beta_negamax(board: &mut Vec<Vec<i32>>, player: i32, depth: i32, mut alpha: i32, beta: i32) -> (i32, i32, i32) {
    let mut best_pos: Option<(i32, i32)> = None;
    let mut best_score = std::i32::MIN;
    
    if depth == 0  || winner_board(board, player) {
        // Evaluate the board using a heuristic function
        let score = evaluate_board(board, player);
        return (-1, -1, score);
    }
    
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            // Heuristic for candidate selection
            if is_candidate(board, x as i32, y as i32, player) {
                // Play the move
                board[x][y] = player;
                
                // Evaluate the move recursively
                let (_, _, score) = alpha_beta_negamax(board, if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE}, depth - 1, -beta, -alpha);
                let score = -score;
                
                // Undo the move
                board[x][y] = UNPLAYED_STATE;
                
                // Check if this move is the best so far
                if score > best_score {
                    best_score = score;
                    best_pos = Some((x as i32, y as i32));
                }
                
                // Apply alpha-beta pruning
                alpha = if best_score > alpha {best_score} else {alpha};
                if alpha >= beta {
                    // TODO: Protect from error return (only call if a piece is placable)
                    return (best_pos.unwrap().0, best_pos.unwrap().1, best_score);
                }
            }
        }
    }
    // TODO: Protect from error return (only call if a piece is placable)
    return (best_pos.unwrap().0, best_pos.unwrap().1, best_score);
}

fn winner_board(board: &mut Vec<Vec<i32>>, player : i32) -> bool {
    
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

fn evaluate_board(board: &mut Vec<Vec<i32>>, player : i32) -> i32 {
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

fn is_candidate(board: &Vec<Vec<i32>>, x: i32, y: i32, player: i32) -> bool {
    if is_legal(board, x, y, player) {
        for n in get_neighbours(x, y, board[0].len() as i32) { 
            if board[n.0 as usize][n.1 as usize] != UNPLAYED_STATE {
                return true;
            }
        }
    }
    false
}