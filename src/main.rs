use gomoku::{*, data::save_tt_table};

pub fn train_heuristic() {
    let mut data = AppState::default();
    data.board_size = 10;
    let mut turn = 0;
    let _ = data.reset();
    data.game_mode = "AIvAI".into();
    let mut scores = Vec::new();
    loop {
        if data.board.game_over(data.turn) {
            break;
        }
        let (x, y, score) = get_best_move(&mut data); 
        scores.push(score);
        let mut m = BoardMove::new(x, y, data.turn);
        if data.board.is_legal_move(x, y, data.turn) {
            m.set(&mut data.board);
        }
        data.turn = data.board.get_opponent(data.turn);
        turn += 1;
        println!("turn: {}", turn);
    }
    save_tt_table(&mut data.tt);
    let sum : i32 = scores.iter().sum();
    let avg = sum as f32 / scores.len() as f32;
    println!("Winner is player {} after {} turns", data.board.return_winner(), turn);
    println!("Average score: {}", avg);
}



pub fn main() {

    let args: Vec<String> = std::env::args().collect();

    for i in 1..args.len() {
        if args[i] == "--train" || args[i] == "-t" {
            train_heuristic();
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
