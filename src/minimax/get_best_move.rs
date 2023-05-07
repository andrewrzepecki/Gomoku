use crate::*;

pub fn get_best_move(board: &mut Board, player: Players) -> (usize, usize) {
    
    let alpha = -100000;
    let beta = 100000;

    let best_move = alpha_beta_negamax(board, player, DEPTH, alpha, beta);
    // let best_move = mtdf(&mut data.board, data.turn, DEPTH, &mut data.tt, 0);
    // println!("{}", best_move.2);
    println!("{}", best_move.2);
    return (best_move.0, best_move.1)
}