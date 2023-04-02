use crate::*;

/*
    Main evaluate board heuristics: Add score for each pattern on board.
 */

pub fn evaluate_board(board: &mut Vec<Vec<i32>>, player: i32) -> i32 {
    let mut score: i32 = 1;
    
    let winner = is_winner_board(board);
    if winner != UNPLAYED_STATE {
        score += if player == winner {1000000} else {-1000000};
    }
    return score
}


/*
    Candidate proposal: Return only viable move candidates in order of evaluate_board
    result.
 */

pub fn is_candidate(board: &Vec<Vec<i32>>, x: i32, y: i32, player: i32) -> bool {
    let size = board[0].len() as i32;
    if is_legal(board, x, y, player) {
        for n in get_neighbours(x, y, size) { 
            if board[n.0 as usize][n.1 as usize] != UNPLAYED_STATE {
                return true;
            }
        }
    }
    false
}


pub fn get_candidates(board: &mut Vec<Vec<i32>>, player: i32) ->  Vec<(i32, i32)> {
    
    // x, y, score
    let mut candidates : Vec<(i32, i32, i32)> = Vec::new();
    let size = board[0].len();

    for x in 0..size {
        for y in 0..size {
            if is_candidate(board, x as i32, y as i32, player) {
                board[x][y] = player;
                let score = evaluate_board(board, player);
                candidates.push((x as i32, y as i32, score));
                board[x][y] = UNPLAYED_STATE;
            }
        }
    }
    // Sort candidates based on score.
    candidates.sort_by(|a, b| a.2.cmp(&b.2)); 
    let sorted_candidates = candidates
        .iter()
        .map(|&(first, second, _)| (first, second))
        .collect::<Vec<(i32, i32)>>();
    return sorted_candidates;
}