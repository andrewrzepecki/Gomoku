use gomoku::*;
use std::time::Instant;

pub fn main() {
    
    let initial_state = AppState {
        
        label : "Test Game".into(),
        board_size : BOARDSIZE,
        turn : 1,
        player1_color : 0,
        player2_color : 1,
        board : Vec::new(),
        captures: Vec::from([0,0]),
        winner : 0,
        game_mode : "PvP".into(),
        colors : Vec::from(
            [
                Color::BLACK, 
                Color::WHITE, 
                Color::BLUE,
                Color::RED, 
                Color::GREEN, 
                Color::YELLOW, 
                Color::SILVER
            ]
        ),
        color_names : Vec::from(
            [
                "BLACK".into(), 
                "WHITE".into(), 
                "BLUE".into(), 
                "RED".into(), 
                "GREEN".into(), 
                "YELLOW".into(), 
                "SILVER".into()
            ]
        ),
        last_move_time : Instant::now(),
    };
    
    let window = WindowDesc::new(build_menu()).title(LocalizedString::new("Gomoku Settings"));
    let launcher = AppLauncher::with_window(window);
    
    launcher
        .launch(initial_state)
        .expect("launch failed");
}
