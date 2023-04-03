use crate::*;

/*
    Main evaluate board heuristics: Add score for each pattern on board.
 */

pub fn evaluate_board(board: &mut Board, player: i32) -> i32 {
    
    let mut score: i32 = 0;
    let opp_player = board.get_opponent(player);
    let winner = board.return_winner();
    
    // Score Winner
    if winner != UNPLAYED_STATE {
        score += if player == winner {1000000} else {-1000000};
    }
    // Score Captures
    score += board.captures[(player - 1) as usize] * 100000;
    score += board.captures[(opp_player - 1) as usize] * -100000;

    
    // Score Open Fours (LiveFour)
    if live_four(board, player) == 1 {
        score += 150000;
    }
    if live_four(board, opp_player) == 1 {
        score += -150000;
    }

    // Score Blocked on one side Fours (DeadFour)
    // TODO
    if live_three(board, player) >= 2 || dead_four(board, player) == 2
        || (dead_four(board, player) == 1) && (live_three(board, player) == 1) {
        score += 100000;
    }
    return score
}

pub fn live_four(board: &mut Board, player: i32) -> i32 {
    board.count_free_closed_alignements_of(4, player)
}

pub fn dead_four(board: &mut Board, player: i32) -> i32 {
    board.count_blocked_closed_alignements_of(4, player)
}

pub fn live_three(board: &mut Board, player: i32) -> i32 {
    let count = board.count_free_open_alignements_of(4, player)
        + board.count_free_open_alignements_of(3, player)
        + board.count_free_closed_alignements_of(3, player);
    count
}

pub fn dead_three(board: &mut Board, player: i32) -> i32 {
    let count = board.count_blocked_open_alignements_of(3, player)
        + board.count_blocked_closed_alignements_of(3, player);
    count
}

/*
    Candidate proposal: Return only viable move candidates in order of evaluate_board
    result.
 */

pub fn evaluate_candidate(board: &mut Board, x: i32, y: i32, player: i32) -> i32 {
    let mut score = 0;
    let opp_player = board.get_opponent(player);
    for n in board.get_neighbours(x, y) {
        if board[(n.0, n.1)] == opp_player {
            let (xd, yd) = board.get_delta((x, y), (n.0, n.1));
            let mut i = 1;
            score += 1; 
            while board.is_valid(n.0 + (xd * i), n.1 + (yd * i)) && board[(n.0 + (xd * i), n.1 + (yd * i))] == opp_player {
                i += 1;
                score += 1;
            }
        }
    }
    println!("{}",score);
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
                let candidate = BoardMove::new(x, y, player);
                let score = evaluate_candidate(board, x, y, player);
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