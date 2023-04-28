use gomoku::*;

pub fn main() {

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
