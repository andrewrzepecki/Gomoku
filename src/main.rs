use gomoku::*;



pub fn main() {

    let args: Vec<String> = std::env::args().collect();
    
    // Collect args
    for i in 1..args.len() {
        if args[i] == "--train" || args[i] == "-t" {
            //train_heuristic();
            return;
        }
    }


    let initial_state = AppState::default();

    let window = WindowDesc::new(main_view())
        .title(LocalizedString::new("Gomoku"))
        .window_size(Size::new(600.0, 450.0)
    );


    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}
