use crate::*;

// Implementation of a Board move to easily play and unplay moves.
//
#[derive(Clone)]
pub struct BoardMove {
    pub x : i32,
    pub y : i32,
    pub player : Players,
    pub to_remove: Vec<(i32, i32)>,
    pub set : bool,
    pub score : i32,
}

//impl BoardMove {
//    pub fn new(x: i32, y: i32, player: Players) -> BoardMove {
//        BoardMove {
//            x : x,
//            y : y,
//            player : player,
//            to_remove : Vec::new(),
//            set : false,
//            score : 0, 
//        }
//    }
//
//    pub fn set(&mut self, board: &mut Board) {
//        if !self.set {
//            self.to_remove = board.return_captured(self.x, self.y, self.player);
//            for &(x, y) in &self.to_remove {
//                board[(x, y)] = UNPLAYED_STATE;
//                board.captures[self.player as usize] += 1;
//            }
//            board[(self.x, self.y)] = self.player as i32;
//            self.set = true;
//        }
//    }
//
//    pub fn unset(&mut self, board: &mut Board) {
//        if self.set {
//            for &(x, y) in &self.to_remove {
//                board[(x, y)] = board.get_opponent(self.player) as i32;
//                board.captures[self.player as usize] -= 1;
//            }
//            board[(self.x, self.y)] = UNPLAYED_STATE;
//            self.set = false;
//        }
//    }
//}

// Main Gomoku Board Logic
#[derive(Clone, Data)]
pub struct Board {
    /*
        Main Board Data Structure using a fixed sized array of u64's:
            00 => UnPlayedState,

     */
    pub size : usize,
    pub bpc : u32,
    pub bpr : u32,
    #[data(eq)]
    pub boards: [u64; BOARDSIZE],
    #[data(eq)]
    pub captures : [u64; 2],
    //#[data(eq)]
    //pub score_table : HashMap<String, (i32, bool)>,
}

//impl Index<(usize, usize)> for Board {
//    type Output = u64;
//
//    fn index(&self, index: (usize, usize)) -> &u64 {
//        let bit_pos = index.0 as u32 * self.bpc;
//        let row = index.1 as u32 * self.bpr;
//        let cell_state = (self.boards[index.1] >> (bit_pos + row % 64)) & 0b11;
//        return &cell_state;

//    }
//}
//
//impl IndexMut<(usize, usize)> for Board {
//    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
//        let bit_pos = index.0 as u32 * self.bpc;
//        let row = index.1 as u32 * self.bpr;
//        let mut cell_state = (self.boards[index.1] >> (bit_pos + row % 64)) & 0b11;
//        &mut cell_state
//    }
//}

impl Board {
    
    pub fn new(size: usize) -> Board {
        Board {
            size : size,
            bpc : 2,
            bpr : size as u32 * BOARDSIZE as u32,
            boards : [0; BOARDSIZE],
            captures : [0; 2],
        }
    }

    pub fn get_state(&self, x: usize, y: usize) -> Players {
        match self.get(x, y) {
            0b00 => Players::Unplayed,
            0b01 => Players::PlayerOne,
            0b10 => Players::PlayerTwo,
            _ => Players::Unplayed,
        }
    }

    // Unsafe function
    pub fn get(&self, x: usize, y: usize) -> u64 {
        let bit_pos = x as u32 * self.bpc;
        let cell_state = (self.boards[y] >> bit_pos) & 0b11;
        cell_state
    }

    // Unsafe function
    pub fn set(&mut self, x: usize, y: usize, player: Players) {
        let mut to_set : u64 = 0u64;
        match player {
            Players::Unplayed => to_set = 0u64,
            Players::PlayerOne => to_set = 1u64,
            Players::PlayerTwo => to_set = 2u64,
        }
        let bit_pos = x as u32 * self.bpc;
        let mask = to_set << bit_pos;
        self.boards[y] |= mask;
    }

    pub fn is_legal(&mut self, x: usize, y: usize, player : Players) -> bool {
        self.is_valid(x, y) && self.is_free(x, y) && !self.is_illegal_capture(x, y, player)
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < BOARDSIZE && y < BOARDSIZE
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        self.get_state(x, y) == Players::Unplayed
    }
    
    pub fn is_illegal_capture(&self, x: usize, y: usize, player: Players) -> bool {
        for n in self.get_neighbors(x, y) {
            if self.get_state(n.0, n.1) == player {
                
            }
        }
        false
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
    //pub fn return_captured(&self, x : i32, y : i32, player : Players) -> Vec<(i32, i32)> {
    //    let opp_player = self.get_opponent(player);
//
    //    for n in self.get_neighbours(x, y) {
    //        if self[(n.0, n.1)] == opp_player  as i32 {
    //            let x_sym = x + ((n.0 - x) * 2);
    //            let y_sym = y + ((n.1 - y) * 2);
    //            let x_opp = x + ((n.0 - x) * 3);
    //            let y_opp = y + ((n.1 - y) * 3);
    //            if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
    //                if self[(x_sym, y_sym)] == opp_player as i32 && self[(x_opp, y_opp)] == player as i32 {
    //                    return vec![(x_sym, y_sym), ((n.0, n.1))];
    //                }
    //            }
    //        }
    //    }
    //    return vec![];
    //}
    
    //pub fn is_illegal_capture(&self, x: i32, y: i32, player: Players) -> bool {
    //    let opp_player = self.get_opponent(player);
    //    for n in self.get_neighbours(x, y) {
    //        if self[(n.0, n.1)] == player as i32 {
    //            let x_sym = x + ((n.0 - x) * 2);
    //            let y_sym = y + ((n.1 - y) * 2);
    //            let x_opp = x + ((n.0 - x) * -1);
    //            let y_opp = y + ((n.1 - y) * -1);
    //            if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
    //                if self[(x_sym, y_sym)] == opp_player as i32 && self[(x_opp, y_opp)] == opp_player as i32 {
    //                    return true;
    //                }
    //            }
    //        }
    //    }
    //    return false;
    //}

    // check if moves triggers an illegal double free three.
    //pub fn get_opponent(&self, player: Players) -> Players {
    //    if player == Players::PlayerOne {Players::PlayerTwo} else {Players::PlayerOne} 
    //}

    //pub fn get_delta(&self, p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    //    let dx = p2.0 - p1.0;
    //    let dy = p2.1 - p1.1;
    //    return (dx, dy);
    //}

}

