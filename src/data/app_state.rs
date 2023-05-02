use crate::*;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub window_name: String,
    pub current_view: i32,
    pub board_size: usize,
    pub board : Board,
    #[data(eq)]
    pub turn : Players,
    #[data(eq)]
    pub captures : [u64; 2],
    #[data(eq)]
    pub winner: Option<Players>,
    #[data(eq)]
    pub player_colors : Vec<i32>,
    #[data(eq)]
    pub game_state : GameState,
    #[data(eq)]
    pub game_mode : GameMode,
    #[data(eq)]
    pub colors : Vec<Color>,
    #[data(eq)]
    pub color_names : Vec<String>,
    pub cursor : Cursor,
    pub last_move_duration : Duration,
    pub last_move_time : Instant,
    #[data(eq)]
    pub is_ai : Vec<bool>,
    pub sugested : Option<(i32, i32)>,
}


impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_name : "Gomoku".into(),
            current_view : GameState::Menu as i32,

            board_size : BOARDSIZE,
            board: Board::new(BOARDSIZE),
            turn : Players::PlayerOne,
            captures : [0, 0],
            winner : None,
            player_colors : vec![0, 1],
            game_state : GameState::Menu,
            game_mode : GameMode::PvAI,
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
            cursor : Cursor::Arrow,
            last_move_duration : Instant::now().duration_since(Instant::now()),
            last_move_time : Instant::now(),
            is_ai : Vec::from([false, false]),
            sugested : None,
        }
    }   
}

impl AppState {
    pub fn reset(&mut self) {
        self.last_move_duration = Instant::now().duration_since(Instant::now());
        self.board = Board::new(BOARDSIZE);
        self.last_move_time = Instant::now();
        self.sugested = None;
        self.turn = Players::PlayerOne;
        self.winner = None;
    }

    pub fn change_cursor(&mut self, player : Players) {
        
        if player == Players::PlayerOne {
            self.cursor = Cursor::Arrow;
        }
        else if player == Players::PlayerTwo {
            self.cursor = Cursor::Crosshair;
        }
        else {
            self.cursor =  Cursor::NotAllowed;
        }

        // self.cursor = match self.cursor {
        //     Cursor::Arrow => Cursor::IBeam,
        //     Cursor::IBeam => Cursor::Pointer,
        //     Cursor::Pointer => Cursor::Crosshair,
        //     Cursor::Crosshair => Cursor::NotAllowed,
        //     Cursor::NotAllowed => Cursor::ResizeLeftRight,
        //     Cursor::ResizeLeftRight => Cursor::ResizeUpDown,
        //     Cursor::ResizeUpDown => {
        //         if let Some(custom) = &self.custom {
        //             custom.clone()
        //         } else {
        //             Cursor::Arrow
        //         }
        //     }
        //     Cursor::Custom(_) => Cursor::Arrow,
        //     _ => Cursor::Arrow,
        // };
    }
}