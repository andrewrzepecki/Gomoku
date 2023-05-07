use crate::*;

// Main Gomoku Board Logic
#[derive(Clone, Data)]
pub struct Board {
    /*
        Main Board Data Structure using a fixed sized array of u64
    */
    pub size : usize,
    pub bpc : u32,
    pub bpr : u32,
    #[data(eq)]
    pub boards: [u64; BOARDSIZE],
    #[data(eq)]
    pub captures : [u64; 2],
    #[data(eq)]
    pub pattern_table : HashMap<String, HashMap<u64, (usize, i32, bool)>>,
    #[data(eq)]
    pub inverted_table : HashMap<String, HashMap<u64, (usize, i32, bool)>>,
}

impl Board {
    
    pub fn new() -> Board {
        Board {
            size : BOARDSIZE,
            bpc : 2,
            bpr : BOARDSIZE as u32 * BOARDSIZE as u32,
            boards : [0; BOARDSIZE],
            captures : [0; 2],
            pattern_table : make_pattern_table(),
            inverted_table : make_inverted_table(),
        }
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < BOARDSIZE && y < BOARDSIZE
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        self.get_state(x, y) == Players::Unplayed
    }

    pub fn get_state(&self, x: usize, y: usize) -> Players {
        get_u64_state(self.boards[y], x)
    }
    
    pub fn set_state(&mut self, x: usize, y: usize, player: Players) {
        set_u64_state(&mut self.boards[y], x, player);
    }

    pub fn get_player_patterns(&self, player: Players) -> &HashMap<String, HashMap<u64, (usize, i32, bool)>> {
        match player {
            Players::PlayerOne => &self.pattern_table,
            Players::PlayerTwo => &self.inverted_table,
            Players::Unplayed => &self.pattern_table,
        }
    }

    pub fn move_is_winner(&mut self, x: usize, y: usize, player : Players) -> bool {

        let mut winner = false;
        self.set_state(x, y, player);
        if self.scan_board(&(*self.get_player_patterns(player))["five_in_a_row"], player) > 0 {
            winner = true;
        }
        if self.captures[player as usize] >= MAX_CAPTURES  as u64 {
            winner = true;
        }
        return winner;
    }

    pub fn move_is_legal(&mut self, x: usize, y: usize, player : Players) -> bool {
        self.is_valid(x, y) &&
        self.is_free(x, y) &&
        !self.is_illegal_capture(x, y, player) &&
        self.move_is_illegal_free_three(x, y, player) < 2
    }

    pub fn is_illegal_capture(&self, x: usize, y: usize, player: Players) -> bool {
        
        let opp = get_opponent(player);
        for n in self.get_neighbors(x, y) {
            if self.get_state(n.0, n.1) == player {
                let (dx, dy) = self.get_delta((x, y), n);
                let x_sym = (x as i32 + (dx * 2)) as usize;
                let y_sym = (y as i32 + (dy * 2)) as usize;
                let x_opp = (x as i32 + (dx * -1)) as usize;
                let y_opp = (y as i32 + (dy * -1)) as usize;
                if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
                    if self.get_state(x_sym, y_sym) == opp && self.get_state(x_opp, y_opp) == opp {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn move_is_illegal_free_three(&mut self, x: usize, y: usize, player: Players) -> i32 {
        self.set_state(x, y, player);
        let patterns = &(*self.get_player_patterns(player))["free_threes"];
        let count = self.scan_board(patterns, player);
        self.set_state(x, y, Players::Unplayed);
        return count;
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {

        let size = self.size;
        (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .filter_map(move |(dx, dy)| {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                if new_x < size && new_y < size {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
    }

    pub fn return_captured(&self, x: usize, y: usize, player : Players) -> [[i8; 2]; 2] {
        
        // Only return first occurrence of captured pieces.
        // Todo: Change captured data struct to return multiple captures.
        let mut captured = [[-1i8; 2]; 2];
        let opp = get_opponent(player);
        for n in self.get_neighbors(x, y) {
            if self.get_state(n.0, n.1) == opp {
                let (dx, dy) = self.get_delta((x, y), n);
                let x_opp = (x as i32 + (dx * 2)) as usize;
                let y_opp = (y as i32 + (dy * 2)) as usize;
                let x_sym = (x as i32 + (dx * 3)) as usize;
                let y_sym = (y as i32 + (dy * 3)) as usize;
                if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
                    if self.get_state(x_sym, y_sym) == player && self.get_state(x_opp, y_opp) == opp {
                        captured = [[n.0 as i8, n.1 as i8], [x_opp as i8, y_opp as i8]];
                        return captured;
                    }
                }
            }
        }
        captured
    }

    pub fn scan_position(&self, x: usize, y: usize, pattern: u64, len: usize, is_live : bool) -> i32 {

        let mut count = 0;
        if self.get_state(x, y) == get_u64_state(pattern, 0) {
            for n in self.get_neighbors(x, y) {
                if self.get_state(n.0, n.1) == get_u64_state(pattern, 1) {
                    let mut is_match = true;
                    if is_live {
                        is_match = false;
                        for (ox, oy) in self.get_next(1, n, (x, y)) {
                            if self.is_valid(ox, oy) && self.get_state(ox, oy) == Players::Unplayed {
                                is_match = true;
                            }
                        }
                    }
                    let mut i = 2;
                    for (nx, ny) in self.get_next((len - 2) as i32, (x, y), n) {
                        if !self.is_valid(nx, ny) || self.get_state(nx, ny) != get_u64_state(pattern, i) {
                            is_match = false;
                            break;
                        }
                        i += 1;
                    }
                    if is_match && i == len {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
    pub fn scan_board(&self, patterns: &HashMap<u64, (usize, i32, bool)>, player: Players) -> i32 {
        let mut total_score = 0.0;
        
        for x in 0..self.size {
            for y in 0..self.size {
                if self.get_state(x, y) == player {
                    for (pattern, (len, score, is_live)) in patterns {
                        let count = self.scan_position(x, y, *pattern, *len, *is_live);
                        let mut sym_pattern = *pattern;
                        let mut sym_len = *len;
                        if *is_live {
                            sym_pattern <<= self.bpc;
                            sym_len += 1;
                        }
                        total_score += if is_symmetrical(sym_pattern, sym_len) { count as f64 / 2.0 * (*score as f64) } else { (count * score) as f64 };
                    }
                }
            }
        }
        total_score as i32
    }

    pub fn get_delta(&self, p1: (usize, usize), p2: (usize, usize)) -> (i32, i32) {
        let dx = p2.0 as i32 - p1.0 as i32;
        let dy = p2.1 as i32 - p1.1 as i32;
        return (dx, dy);
    }

    // Expensive function as allocates to heap, could return fixed size array with a limit, board size>?
    // todo : change return type to static array make a limit to 6.
    pub fn get_next(&self, number: i32, origin: (usize, usize), neighbor: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut next = Vec::new();
        let (dx, dy) = self.get_delta(origin, neighbor);
        for i in 0..number {
            let x = (origin.0 as i32 + (dx * (i + 2) as i32)) as usize;
            let y = (origin.1 as i32 + (dy * (i + 2) as i32)) as usize;
            next.push((x, y));
        }
        next.into_iter()
    }
    pub fn print(&self) {
        for i in 0..self.size {
            println!("{:#038b}", self.boards[i]);
        }
        println!("");
    }
}
