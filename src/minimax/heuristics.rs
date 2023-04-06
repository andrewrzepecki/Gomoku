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
    score
}


pub fn get_final_score(board: &mut Board, player: i32) -> i32 {
    
    let opp = board.get_opponent(player);
    
    let player_score = evaluate_board(board, player) + capture_score(board, player);
    
    let opp_score = evaluate_board(board, opp) + capture_score(board, opp);
    
    return player_score - ((opp_score as f64) * OPPONENT_WEIGHT).round() as i32;
}

// Exponential untill 100.000
pub fn capture_score(board: &mut Board, player: i32) -> i32 {
    let v = 1.152 * board.captures[(player - 1) as usize] as f64;
    let value = (2.718281828461 as f64).powf(v).round() as i32;
    return value;
}


pub fn evaluate_move(board : &mut Board, player_move: &mut BoardMove) -> i32 {
    
    player_move.set(board);
    let opp = board.get_opponent(player_move.player);

    let player_score = evaluate_board(board, player_move.player) + capture_score(board, player_move.player);
    
    let opp_score = evaluate_board(board, opp) + capture_score(board, opp);
    player_move.unset(board);
    
    return player_score - ((opp_score as f64) * 1.2).round() as i32;

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

pub fn get_random_move(board: &mut Board, player: i32) -> BoardMove {
    
    let moves = board.get_legal_moves(player);
    let mut randomizer = rand::thread_rng();
    let index = randomizer.gen_range(0..moves.len());
    let m = BoardMove::new(moves[index as usize].x, moves[index as usize].y, player);
    return m;
}


pub fn get_moves(board: &mut Board, player: i32, tt: &mut HashMap<String, (i32, i32)>) ->  Vec<BoardMove> {
    
    // Try All Adjacent 
    let mut moves : Vec<BoardMove> = Vec::new();
    let board_hash = board.get_hash(player);
    if let Some(entry) = tt.get(&board_hash) {
        let best_move = BoardMove::new(entry.0, entry.1, player);
        moves.push(best_move.clone());
        if board.is_legal_move(best_move.x, best_move.y, player) {
            return moves;
        }
    }
    
    for x in 0..board.size {
        for y in 0..board.size {            
            if is_candidate(board, x as i32, y as i32, player) {
                let mut candidate = BoardMove::new(x, y, player);
                candidate.score = evaluate_move(board, &mut candidate);
                moves.push(candidate);
            }
        }
    }

    // Sort candidates based on score (Offense / Defense).
    moves.sort_by(|a, b| b.score.cmp(&a.score)); 
    
    for _m in moves.clone() {
        println!("c_score: {}", _m.score);
    }
    println!("");
    if moves.len() > CANDIDATE_SELECT {
        let best_score = moves[0].score;
        let mut offset = 0;
        for i in 0..moves.len() {
            if moves[i].score == best_score {
                offset += 1;
            }
        }
        if offset > CANDIDATE_SELECT {
            moves = moves[0..offset].to_vec();
        }
        else {
            moves = moves[0..CANDIDATE_SELECT].to_vec();
        }
    }
    
    // Add random at end.
    let r_move = get_random_move(board, player);
    if r_move.x != -1 {
        moves.push(r_move);
    }
    return moves;
}
