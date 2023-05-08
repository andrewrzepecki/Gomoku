use crate::*;
/*
    Main evaluate board heuristics: Add score for each pattern on board.
*/

// Final Board Score
pub fn get_board_score(board: &mut Board, player : Players) -> i32 {
    let patterns = &(*board.get_player_patterns(player))["score_table"];
    let score = board.scan_board(patterns, player) + capture_score(board, player);
    score
}

// Exponential until 100.000
pub fn capture_score(board: &mut Board, player: Players) -> i32 {
    let v = 1.152 * board.captures[player as usize] as f64;
    (2.718281828461 as f64).powf(v).round() as i32
}

pub fn get_candidate_score(board: &mut Board, x: usize, y: usize, player: Players) -> i32 {

    let mut total = 0;
    for n in board.get_neighbors(x, y) {
        let mut r_count = 0;
        if board.get_state(n.0, n.1) == player {
            r_count += 1;
            for (nx, ny) in board.get_next(5, (x, y), n) {
                if board.is_valid(nx, ny) {
                    let state = board.get_state(nx, ny);
                    if state != player {
                        break;
                    }
                    r_count += 1;
                }
            }
        }
        let mut l_count = 0;
        for (nx, ny) in board.get_next(5, n, (x, y)) {
            if board.is_valid(nx, ny) {
                let state = board.get_state(nx, ny);
                if state != player {
                    break;
                }
                l_count += 1;
            }
        }
        total = if r_count + l_count > total {r_count + l_count} else {total};
    }
    total
}


pub fn get_candidate_moves(board: &mut Board, player: Players) -> [(usize, usize, i32); CANDIDATE_SELECT] {

    let mut tmp_moves = Vec::new();

    // Try All Adjacent
    for x in 0..board.size {
        for y in 0..board.size {
            if board.move_is_legal(x, y, player) {
                  for (nx, ny) in board.get_neighbors(x, y) {
                      if board.get_state(nx, ny) != Players::Unplayed {
                          tmp_moves.push((x, y, get_candidate_score(board, x, y, player).max(get_candidate_score(board, x, y, get_opponent(player)))));
                          break;
                      }
                  }
            }
        }
    }
    // Protect stack array return length (if under CANDIDATE_SELECT add randoms)
    tmp_moves.sort_by(|a, b| b.2.cmp(&a.2));
    let mut moves: [(usize, usize, i32); CANDIDATE_SELECT] = tmp_moves
        .into_iter()
        .take(CANDIDATE_SELECT)
        .map(|(x, y, score)| (x, y, score))
        .collect::<Vec<(usize, usize, i32)>>()
        .try_into()
        .unwrap();

    return moves;
}
