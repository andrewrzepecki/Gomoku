use crate::*;

// Implementation of a Board move to easily play and unplay moves.
#[derive(Clone)]
pub struct BoardMove {
    pub x : usize,
    pub y : usize,
    pub player : Players,
    pub to_remove: [[i8; 2]; 2],
    pub set : bool,
    pub score : i32,
}

impl BoardMove {
    pub fn new(x: usize, y: usize, player: Players) -> BoardMove {
        BoardMove {
            x : x,
            y : y,
            player : player,
            to_remove : [[-1i8; 2]; 2],
            set : false,
            score : 0, 
        }
    }

    pub fn set(&mut self, board: &mut Board) {
        if !self.set {
            self.to_remove = board.return_captured(self.x, self.y, self.player);
            for add in self.to_remove {
                if add[0] != -1 {
                    board.set_state(add[0] as usize, add[1] as usize, Players::Unplayed);
                    board.captures[self.player as usize] += 1;
                }
            }
            board.set_state(self.x, self.y, self.player);
            self.set = true;
        }
    }

    pub fn unset(&mut self, board: &mut Board) {
        if self.set {
            for add in self.to_remove {
                if add[0] != -1 {
                    board.set_state(add[0] as usize, add[1] as usize, get_opponent(self.player));
                    board.captures[self.player as usize] -= 1;
                }
            }
            board.set_state(self.x, self.y, Players::Unplayed);
            self.set = false;
        }
    }
}

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
    pub pattern_table : HashMap<String, HashMap<u64, (usize, i32)>>,
    #[data(eq)]
    pub inverted_table : HashMap<String, HashMap<u64, (usize, i32)>>,
}

impl Board {
    
    pub fn new(size: usize) -> Board {
        Board {
            size : size,
            bpc : 2,
            bpr : size as u32 * BOARDSIZE as u32,
            boards : [0; BOARDSIZE],
            captures : [0; 2],
            pattern_table : make_pattern_table(),
            inverted_table : make_inverted_table(),
        }
    }

    pub fn get_state(&self, x: usize, y: usize) -> Players {
        get_u64_state(self.boards[y], x)
    }
    
    pub fn set_state(&mut self, x: usize, y: usize, player: Players) {
        set_u64_state(&mut self.boards[y], x, player);
    }

    pub fn is_legal(&mut self, x: usize, y: usize, player : Players) -> bool {
        self.is_valid(x, y) && self.is_free(x, y) && !self.is_illegal_capture(x, y, player) && !self.is_illegal_double_three(x, y, player)
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < BOARDSIZE && y < BOARDSIZE
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        self.get_state(x, y) == Players::Unplayed
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

    pub fn is_illegal_double_three(&mut self, x: usize, y: usize, player : Players) -> bool {
        
        let mut mv = BoardMove::new(x, y, player);
        
        mv.set(self);
        
        let mut patterns = &self.pattern_table["free_threes"];
        if player == Players::PlayerTwo {
            patterns = &self.inverted_table["free_threes"];
        }
        if self.scan_board(patterns) > 1 {
            mv.unset(self);
            return true;
        }
        
        mv.unset(self);
        
        return false;
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
        // Only return first occurance of captured pieces. 
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

    pub fn scan_position(&self, x: usize, y: usize, pattern: u64, len: usize) -> i32 {
        
        let mut count = 0;
        
        if self.get_state(x, y) == get_u64_state(pattern, 0) {
            for n in self.get_neighbors(x, y) {
                if self.get_state(n.0, n.1) == get_u64_state(pattern, 1) {
                    let mut is_match = true;
                    let (dx, dy) = self.get_delta((x, y), n);
                    for i in 2..len {
                        let x_check = (x as i32 + (dx * i as i32)) as usize;
                        let y_check = (y as i32 + (dy * i as i32)) as usize;
                        if !self.is_valid(x_check, y_check) || self.get_state(x_check, y_check) != get_u64_state(pattern, i) {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
    pub fn scan_board(&self, patterns: &HashMap<u64, (usize, i32)>) -> i32 {
        
        let mut total_score = 0.0;
        
        for x in 0..self.size {
            for y in 0..self.size {
                for (pattern, (len, score)) in patterns {
                    let count = self.scan_position(x, y, *pattern, *len);
                    total_score += if is_symetrical(*pattern, *len) {count as f64 / 2.0 * (*score as f64)} else {(count * score) as f64};
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

    pub fn print(&self) {
        for i in 0..self.size {
            println!("{:#038b}", self.boards[i]);
        }
        println!("");
    }
}
