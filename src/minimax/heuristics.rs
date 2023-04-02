use crate::*;

/*
    Main evaluate board heuristics: Add score for each pattern on board.
 */

pub fn evaluate_board(board: &mut Board, player: i32) -> i32 {
    let mut score: i32 = 0;
    
    let winner = board.return_winner();
    if winner != UNPLAYED_STATE {
        score += if player == winner {1000000} else {-1000000};
    }
    return score
}


/*
    Candidate proposal: Return only viable move candidates in order of evaluate_board
    result.
 */

pub fn is_candidate(board: &mut Board, x: i32, y: i32, player: i32) -> bool {
    if board.is_legal_move(x, y, player) {
        for n in board.get_neighbours(x, y) { 
            if board[(n.0, n.1)] != UNPLAYED_STATE {
                return true;
            }
        }
    }
    false
}


pub fn get_moves(board: &mut Board, player: i32) ->  Vec<BoardMove> {
    
    // x, y, score
    let mut moves : Vec<(BoardMove, i32)> = Vec::new();

    for x in 0..board.size {
        for y in 0..board.size {
            
            if is_candidate(board, x as i32, y as i32, player) {
                let mut candidate = BoardMove::new(x, y, player);
                candidate.set(board);
                let score = evaluate_board(board, player);
                candidate.unset(board);
                moves.push((candidate, score));
            }
        }
    }

    // Sort candidates based on score.
    moves.sort_by(|a, b| a.1.cmp(&b.1)); 
    let mut sorted = Vec::new();
    for m in moves {
        sorted.push(m.0);
    }
    return sorted;
}