use rand::Rng;
use crate::*;

/*
    Main evaluate board heuristics: Add score for each pattern on board.
*/

// Final Board Score
pub fn get_board_score(board: &mut Board, player : Players) -> i32 {
    let opp = get_opponent(player);
    let patterns = &(*board.get_player_patterns(player))["score_table"];
    let opp_patterns = &(*board.get_player_patterns(opp))["score_table"];
    let score = board.scan_board(patterns, player) - ((board.scan_board(opp_patterns, opp) as f64 * OPPONENT_WEIGHT) as i32) + capture_score(board, player);
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
        let mut b_count = 0;
        let n_state = board.get_state(n.0, n.1);
        if n_state == player  || n_state == Players::Unplayed {
            match n_state {
                 Players::Unplayed => b_count += 1,
                _ => r_count += 1,
            };
            for (nx, ny) in board.get_next(5, (x, y), n) {
                if board.is_valid(nx, ny) {
                    let state = board.get_state(nx, ny);
                    if state == Players::Unplayed && b_count < 1 {
                        b_count += 1;
                    } else if state != player {
                        break;
                    } else {
                        r_count += 1;
                    }
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

pub fn get_random_coords() -> (usize, usize, i32) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..BOARDSIZE-1);
    let y = rng.gen_range(0..BOARDSIZE-1);
    (x, y, -1)
}

pub fn get_n_random_candidates(board: &mut Board, player: Players, n: usize) -> Vec<(usize, usize, i32)> {

    let mut randoms = Vec::new();
    let mut flag = true;
    while flag {
        let (x, y, score) = get_random_coords();
        if board.move_is_legal(x, y, player)
            && !randoms.iter().any(|v| *v == (x, y, score)) {
            randoms.push((x, y, score));
        }
        if randoms.len() >= n {
            flag = false;
            break;
        }
    }
    randoms
}

fn get_adjacent(board: &mut Board, player: Players) -> Vec<(usize, usize, i32)> {

    let mut moves = Vec::new();
    // Score All Adjacent + 1
    for x in 0..board.size {
        for y in 0..board.size {
            for (nx, ny) in board.get_neighbors(x, y) {
                if board.get_state(nx, ny) != Players::Unplayed && board.move_is_legal(x, y, player) {
                    moves.push((x, y, get_candidate_score(board, x, y, player).max(get_candidate_score(board, x, y, get_opponent(player)))));
                }
            }
        }
    }
    moves
}


pub fn get_candidate_moves(board: &mut Board, player: Players) -> [(usize, usize, i32); CANDIDATE_SELECT] {

    let mut moves = [(42usize, 42usize, -1i32); CANDIDATE_SELECT];
    let mut tmp_moves = get_adjacent(board, player);

    // Sort adjacent candidates...
    tmp_moves.sort_by(|a, b| b.2.cmp(&a.2));
    let len = tmp_moves.len();
    let random_move = get_n_random_candidates(board, player, 1);
    for i in 0..CANDIDATE_SELECT - 1 {
        if i < len { moves[i] = tmp_moves[i]; }
    }
    let index = if len < CANDIDATE_SELECT { len } else { CANDIDATE_SELECT - 1 };
    moves[index] = random_move[0];
    return moves;
}
