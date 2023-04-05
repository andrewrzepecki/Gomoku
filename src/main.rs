use gomoku::*;


pub fn test_heuristic() {
    let mut data = AppState::default();
    let mut turn = 0;
    let mut player = PLAYER1_STATE;
    let pieces = data.reset();
    let mut tt: std::collections::HashMap<String, (i32, i32, i32)> =  std::collections::HashMap::new();
    let mut flag = true;
    data.game_mode = "AIvAI".into();
    while flag {
        if data.board.game_over(player) {
            break;
        }
        let (x, y, score) = alpha_beta_minimax(
            &mut data.board,
            player,
            DEPTH,
            true,
            -1000000,
            1000000, 
            &mut tt
        );
        println!("{} {}", x, y);
        let mut m = BoardMove::new(x, y, player);
        if data.board.is_legal_move(x, y, player) {
            m.set(&mut data.board);
        }
        player = data.board.get_opponent(player);
        turn += 1;
        println!("{}", turn);
    }
    println!("Winner is player {} after {} turns", data.board.return_winner(), turn);
}



pub fn main() {

    let args: Vec<String> = std::env::args().collect();

    for i in 1..args.len() {
        if args[i] == "--train" || args[i] == "-t" {
            test_heuristic();
            return;
        }
    }



    let initial_state = AppState::default();
    
    let window = WindowDesc::new(build_menu())
        .title(LocalizedString::new("Gomoku Settings"))
        .resizable(false)
        .window_size(Size::new(600.0, 450.0)
    );



    let launcher = AppLauncher::with_window(window);
        launcher
        .launch(initial_state)
        .expect("launch failed");
}
