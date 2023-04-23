use crate::*;


#[derive(Clone, PartialEq, Eq, Copy)]
pub enum  GameState {
    Menu,
    Game,
    GameOver,    
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum  GameMode {
    PvP,
    PvAI,
    AIvAI,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::PvAI
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum  Players {
    PlayerOne,
    PlayerTwo,
}

impl Players {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl Default for Players {
    fn default() -> Self {
        Players::PlayerOne
    }
}


///    Main App State For Gomoku App,
///'''
///    let initial_state = AppState::Default()
///    initial_state.reset()
///'''


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub window_name: String,
    pub current_view: i32,



    pub board_size: i32,
    pub board : Board,
    #[data(eq)]
    pub turn : Players,
    #[data(eq)]
    pub captures : Vec<i32>,
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
            captures : vec![0, 0],
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
        self.last_move_time = Instant::now();
        self.sugested = None;
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
        /* 
        self.cursor = match self.cursor {
            Cursor::Arrow => Cursor::IBeam,
            Cursor::IBeam => Cursor::Pointer,
            Cursor::Pointer => Cursor::Crosshair,
            Cursor::Crosshair => Cursor::NotAllowed,
            Cursor::NotAllowed => Cursor::ResizeLeftRight,
            Cursor::ResizeLeftRight => Cursor::ResizeUpDown,
            Cursor::ResizeUpDown => {
                if let Some(custom) = &self.custom {
                    custom.clone()
                } else {
                    Cursor::Arrow
                }
            }
            Cursor::Custom(_) => Cursor::Arrow,
            _ => Cursor::Arrow,
        };*/
    }
}