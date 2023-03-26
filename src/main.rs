use gomoku::*;

pub fn main() {
    
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
