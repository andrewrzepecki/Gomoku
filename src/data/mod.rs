use crate::*;


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub label: String,
    pub board_size : i32,
    pub turn : i32,
    pub player1_color : i32,
    pub player2_color : i32,
    pub board : Board,
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
    pub winner_opened : bool,
    #[data(eq)]
    pub tt : HashMap<String, (i32, i32, i32)>,
    pub is_test : bool,
    pub player1_score : i32,
    pub player2_score : i32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            label : "Test Game".into(),
            board_size : BOARDSIZE,
            turn : PLAYER1_STATE,
            player1_color : 0,
            player2_color : 1,
            board : Board::new(BOARDSIZE),
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
            winner_opened : false,
            tt : load_tt_table(),
            is_test : false,
            player1_score : 0,
            player2_score : 0,
        }
    }
}

impl AppState {
    pub fn reset(&mut self) -> Vector<BoardPiece> {
        let pieces = build_pieces(self.board_size);
        self.board = Board::new(self.board_size);
        self.turn = PLAYER1_STATE;
        self.captures = Vec::from([0,0]);
        self.winner = UNPLAYED_STATE;
        self.last_move_duration = Instant::now().duration_since(Instant::now());
        self.last_move_time = Instant::now();
        self.is_ai = Vec::from([if self.game_mode == "AIvAI" {true} else {false}, if self.game_mode == "PvP" {false} else {true}]);
        self.sugested = None;
        self.winner_opened = false;
        pieces
    }

    pub fn test(&mut self) -> Vector<BoardPiece> {
        self.board_size = 15;
        self.game_mode = "PvP".into();
        self.is_test = true;
        self.player1_score = 0;
        self.player2_score = 0;
        return self.reset();
    }
}


fn load_tt_table() -> HashMap<String, (i32, i32, i32)> {
    if Path::new(&TT_PATH).exists() {
        println!("Found Transposition Table!");
        let file = File::open(TT_PATH).unwrap();
        let reader = BufReader::new(file);
        let tt = serde_json::from_reader(reader).unwrap();
        return tt;
    }
    else {
        let tt = HashMap::new();
        return tt;
    }
}

pub fn save_tt_table(tt: &mut HashMap<String, (i32, i32, i32)>) {
    // Open a file for writing
    let file = File::create(TT_PATH).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &tt).unwrap();
}