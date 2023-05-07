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
    pub is_ai : [bool; 2],
    pub suggested : Option<(i32, i32)>,
    pub is_test : bool,
    pub candidate_score: i32,
    pub is_playing : bool,
}


impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_name : "Gomoku".into(),
            current_view : GameState::Menu as i32,

            board_size : BOARDSIZE,
            board: Board::new(),
            turn : Players::PlayerOne,
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
            is_ai : [false, false],
            suggested : None,
            is_test : false,
            candidate_score : 0,
            is_playing : false,
        }
    }   
}

impl AppState {
    pub fn reset(&mut self) {

        self.last_move_duration = Instant::now().duration_since(Instant::now());
        self.board = Board::new();
        self.last_move_time = Instant::now();
        self.suggested = None;
        self.turn = Players::PlayerOne;
        self.winner = None;
        self.is_playing = false;
    }

    pub fn change_cursor(&mut self, legal : bool) {
        self.cursor = match legal {
            true => Cursor::Arrow,
            false => Cursor::NotAllowed
        };
     }

    pub fn update_board(&mut self, x: usize, y: usize) {

        self.last_move_duration = Instant::now().duration_since(self.last_move_time);
        self.last_move_time = Instant::now();
        let mut m = BoardMove::new(x, y, self.turn);
        m.set(&mut self.board);
        if self.board.move_is_winner(x, y, self.turn) && !self.is_test {
            self.winner = Some(self.turn);
            self.game_state = GameState::GameOver;
            self.current_view = self.game_state as i32;
            self.turn = Players::Unplayed;
        }
        if !self.is_test {
            self.turn = get_opponent(self.turn);
        }
    }
}