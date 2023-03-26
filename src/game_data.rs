use druid::{Data, Lens, Color};
use std::time::{Duration, Instant};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub label: String,
    pub board_size : i32,
    pub turn : i32,
    pub player1_color : i32,
    pub player2_color : i32,
    #[data(eq)]
    pub board : Vec<Vec<i32>>,
    #[data(eq)]
    pub captures : Vec<i32>,
    pub winner : i32,
    pub game_mode : String,
    #[data(eq)]
    pub colors : Vec<Color>,
    #[data(eq)]
    pub color_names : Vec<String>,
    pub last_move_duration : Duration,
    pub last_move_time : Instant,
    #[data(eq)]
    pub is_ai : Vec<bool>,
}