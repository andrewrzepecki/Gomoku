use crate::*;
/*
    Main evaluate board heuristics: Add score for each pattern on board.
*/

pub fn evaluate_board(board: &mut Board, player: i32) -> i32 {
    
    let mut score: i32 = 0;
    let opp_player = board.get_opponent(player);
    
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
    //score += board.captures[(player - 1) as usize] * 1000;
    score
}


pub fn get_final_score(board: &mut Board, player: i32) -> i32 {
    let opp = board.get_opponent(player);
    let player_score = evaluate_board(board, player) + capture_score(board, player);
    let opp_score = evaluate_board(board, opp) + capture_score(board, opp);
    return player_score - ((opp_score as f64) * OPPONENT_WEIGHT).round() as i32;
}


pub fn capture_score(board: &mut Board, player: i32) -> i32 {
    return board.captures[(player - 1) as usize] * 1500;
}


pub fn evaluate_move(board : &mut Board, player_move: &mut BoardMove) -> i32 {
    
    let c_score = evaluate_board(board, player_move.player) + capture_score(board, player_move.player);
    player_move.set(board);
    let move_score = evaluate_board(board, player_move.player) + capture_score(board, player_move.player);
    player_move.unset(board);

    return c_score - move_score;
}

pub fn is_candidate(board: &mut Board, x: i32, y: i32, player: i32) -> bool {
    if board.is_legal_move(x, y, player) {
        for n in board.get_neighbours(x, y) { 
            if board[(n.0, n.1)] != UNPLAYED_STATE {
                return true;
            }
            //else if board.is_legal_move(n.0, n.1, player){
            //    for adj in board.get_neighbours(n.0, n.1) {
            //        if board[(adj.0, adj.1)] != UNPLAYED_STATE {
            //            return true;
            //        }
            //    }
            //} 
        }
    }
    false
}

pub fn _get_moves(board: &mut Board, player: i32) ->  Vec<BoardMove> {
    
    // Try All Adjacent 
    let mut offensive_moves : Vec<(BoardMove, i32)> = Vec::new();
    let mut defensive_moves : Vec<(BoardMove, i32)> = Vec::new();
    for x in 0..board.size {
        for y in 0..board.size {            
            if is_candidate(board, x as i32, y as i32, player) {
                let mut candidate = BoardMove::new(x, y, player);
                let score = evaluate_move(board, &mut candidate);
                if score >= 0 {
                    offensive_moves.push((candidate, score));
                }
                else {
                    defensive_moves.push((candidate, score));
                }
            }
        }
    }

    // Sort candidates based on score (Offense / Defense).
    offensive_moves.sort_by(|a, b| b.1.cmp(&a.1)); 
    let mut o_sorted = Vec::new();

    defensive_moves.sort_by(|a, b| b.1.cmp(&a.1)); 
    let mut d_sorted = Vec::new();

    for m in offensive_moves {
        o_sorted.push(m.0);
    }
    if o_sorted.len() > CANDIDATE_SELECT {
        o_sorted = o_sorted[0..CANDIDATE_SELECT].to_vec();
    }
    
    
    for m in defensive_moves {
        d_sorted.push(m.0);
    }
    if d_sorted.len() > CANDIDATE_SELECT {
        d_sorted = d_sorted[0..CANDIDATE_SELECT].to_vec();
    }
    
    o_sorted.extend(d_sorted);
    
    // Add random at end.
    let r_move = get_random_move(board, player);
    if r_move.x != -1 {
        o_sorted.push(r_move);
    }
    return o_sorted;
}


pub fn get_random_move(board: &mut Board, player: i32) -> BoardMove {
    
    let mut best_x = -1;
    let mut best_y = -1;
    
    for i in 0..board.size {
        for j in 0..board.size {
            if board.is_legal_move(i, j, player) {
                let mut count = 0;
                for (x, y) in board.get_neighbours(i, j) {
                    if board[(x, y)] == UNPLAYED_STATE {
                        count += 1;
                    }
                }
                if count == 8 {
                    best_x = i;
                    best_y = j;
                    for (x, y) in board.get_neighbours(i, j) {
                        if board[(x, y)] == UNPLAYED_STATE {
                            count += 1;
                        }
                    }
                    if count >= 24 {
                        return BoardMove::new(best_x, best_y, player);
                    }
                }
            }
        }
    }
    return BoardMove::new(best_x, best_y, player);
}


pub fn get_moves(board: &mut Board, player: i32) ->  Vec<BoardMove> {
    
    // Try All Adjacent 
    let mut offensive_moves : Vec<(BoardMove, i32)> = Vec::new();
    for x in 0..board.size {
        for y in 0..board.size {            
            if is_candidate(board, x as i32, y as i32, player) {
                let mut candidate = BoardMove::new(x, y, player);
                let score = evaluate_move(board, &mut candidate);
                offensive_moves.push((candidate, score));
            }
        }
    }

    // Sort candidates based on score (Offense / Defense).
    offensive_moves.sort_by(|a, b| a.1.cmp(&b.1)); 
    let mut o_sorted = Vec::new();


    for m in offensive_moves {
        o_sorted.push(m.0);
    }
    if o_sorted.len() > CANDIDATE_SELECT {
        o_sorted = o_sorted[0..CANDIDATE_SELECT].to_vec();
    }
    
    
    // Add random at end.
    let r_move = get_random_move(board, player);
    if r_move.x != -1 {
        o_sorted.push(r_move);
    }
    return o_sorted;
}
