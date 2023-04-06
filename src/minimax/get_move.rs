use crate::*;

pub fn get_best_move(data: &mut AppState) -> (i32, i32, i32) {
    
    let alpha = -100000;
    let beta = 100000;
    
    
    let best_move = alpha_beta_negamax(&mut data.board, data.turn, DEPTH, alpha, beta, &mut data.tt);
    //let best_move = mtdf(&mut data.board, data.turn, DEPTH, &mut data.tt, 0); 
    //let best_move = alpha_beta_minimax(
    //            &mut data.board,
    //            data.turn,
    //            DEPTH,
    //            true,
    //            alpha,
    //            beta, 
    //            &mut data.tt
    //        );
    println!("{}", best_move.2);
    return best_move
}