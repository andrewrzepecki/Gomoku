use crate::*;
/*
    Main evaluate board heuristics: Add score for each pattern on board.
*/

// Exponential until 100.000
pub fn capture_score(board: &mut Board, player: Players) -> i32 {
    let v = 1.152 * board.captures[player as usize] as f64;
    let value = (2.718281828461 as f64).powf(v).round() as i32;
    return value;
}

pub fn get_candidate_score(board: &mut Board, x: usize, y: usize, player: Players) -> i32 {

    let mut r_bcount = 0;
    let mut r_pcount = 0;
    let mut l_bcount = 0;
    let mut l_pcount = 0;
    for n in board.get_neighbors(x, y) {

        for (nx, ny) in board.get_next(5, (x, y), n) {
            if board.is_valid(nx, ny){
                let state = board.get_state(nx, ny);
                if state == get_opponent(player){
                    break;
                }
                else if state == player {
                    r_pcount += 1;
                }
                else {
                    r_bcount += 1;
                }
                if r_bcount > 1 {
                    break;
                }
            }
        }
        for (nx, ny) in board.get_next(5, n, (x, y)) {
            if board.is_valid(nx, ny){
                let state = board.get_state(nx, ny);
                if state == get_opponent(player){
                    break;
                }
                else if state == player {
                    l_pcount += 1;
                }
                else {
                    l_bcount += 1;
                }
                if l_bcount > 1 {
                    break;
                }
            }
        }
    }
    r_bcount + r_pcount + l_bcount + l_pcount
    // let mut total_score = 0;
    // let mut patterns = (*board.get_player_patterns(players))["candidate_score_table"];
    // for (mut pattern, (mut len, score, is_live)) in patterns {
    //     total_score += (board.scan_position(x, y, pattern, len, is_live) * score);
    // }
    // total_score
}

pub fn get_candidate_moves(board: &mut Board, player: Players) -> [(usize, usize, i32); CANDIDATE_SELECT] {

    let mut tmp_moves = Vec::new();

    // Try All Adjacent
    for x in 0..board.size {
        for y in 0..board.size {
            if board.move_is_legal(x, y, player) {
                  for (nx, ny) in board.get_neighbors(x, y) {
                      if board.get_state(nx, ny) != Players::Unplayed {
                          tmp_moves.push((x, y, get_candidate_score(board, x, y, player) + get_candidate_score(board, x, y, get_opponent(player))));
                          break;
                      }
                  }
            }
        }
    }
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
