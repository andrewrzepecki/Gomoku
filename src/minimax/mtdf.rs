use crate::*;


pub fn mtdf(board: &mut Board, player: i32, depth: i32, tt: &mut HashMap<String, (i32, i32)>, f: i32) -> (i32, i32, i32) {
    
    let mut g = f;
    let mut x = -1;
    let mut y = -1;
    let mut upper_bound = std::i32::MAX;
    let mut lower_bound = std::i32::MIN;
    let mut beta: i32;
    
    
    while lower_bound < upper_bound {
        if g == lower_bound {
            beta = g + 1;
        } else {
            beta = g;
        }
        (x, y, g) = alpha_beta_negamax(board, player, depth, beta - 1, beta, tt);
        if g < beta {
            upper_bound = g;
        } else {
            lower_bound = g;
        }
    }
    return (x, y, g);
}