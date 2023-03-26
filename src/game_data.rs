use crate::*;

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
    pub sugested : Option<(i32, i32)>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            label : "Test Game".into(),
            board_size : BOARDSIZE,
            turn : PLAYER1_STATE,
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
            last_move_duration : Instant::now().duration_since(Instant::now()),
            last_move_time : Instant::now(),
            is_ai : Vec::from([false, false]),
            sugested : None,
        }
    }
}

impl AppState {
    pub fn reset(&mut self) -> Vector<BoardPiece> {
        let pieces = build_pieces(self.board_size);
        self.board = build_board(self.board_size);
        self.turn = PLAYER1_STATE;
        self.captures = Vec::from([0,0]);
        self.winner = 0;
        self.last_move_duration = Instant::now().duration_since(Instant::now());
        self.last_move_time = Instant::now();
        self.is_ai = Vec::from([if self.game_mode == "AIvAI" {true} else {false}, if self.game_mode == "PvP" {false} else {true}]);
        self.sugested = None;
        pieces
    }
}