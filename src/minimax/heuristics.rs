use crate::*;

/*
    Main evaluate board heuristics: Add score for each pattern on board.
 */

pub fn evaluate_board(board: &mut Board, player: i32) -> i32 {
    
    let mut score: i32 = 0;
    let opp_player = board.get_opponent(player);
    
    // Score Winner
    for line in board.get_lines() {

        // Scan Every Line for pattern.
        for (pattern, value) in &board.score_table {
            
            // Change pattern to current player values.
            let p = pattern.replace("x", &format!("{}", player));
            let p = p.replace("o", &format!("{}", opp_player));
            score += board.count_line_occurrences(&line, &p) * value.0;
            
            // Non-symetrical
            if value.1 {
                let reversed = p.chars().rev().collect::<String>();
                score += board.count_line_occurrences(&line, &reversed) * value.0;
            }
        }
    }

    // Score Captures
    score += board.captures[(player - 1) as usize] * 10000;
    score
}


/*
    Candidate proposal: Return only viable move candidates in order of evaluate_board
    result.
 */

pub fn evaluate_candidate(board: &mut Board, x: i32, y: i32, player: i32) -> i32 {
    let mut score = 0;
    //let opp_player = board.get_opponent(player);
    for n in board.get_neighbours(x, y) {
        if board[(n.0, n.1)] == player {
            let (xd, yd) = board.get_delta((x, y), (n.0, n.1));
            let mut i = 1;
            score += 1; 
            while board.is_valid(n.0 + (xd * i), n.1 + (yd * i)) && board[(n.0 + (xd * i), n.1 + (yd * i))] == player {
                i += 1;
                score += 1;
            }
        }
    }
    score
}


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