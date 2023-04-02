use crate::*;


// Implementation of a Board move to easily play and unplay moves.
//

pub struct BoardMove {
    pub to_set : (i32, i32),
    pub player : i32,
    pub to_remove: Vec<(i32, i32)>,
    pub set : bool,
}

impl BoardMove {
    pub fn new(x: i32, y: i32, player: i32) -> BoardMove {
        BoardMove {
            to_set : (x, y),
            player : player,
            to_remove : Vec::new(),
            set : false,
        }
    }

    pub fn set(&mut self, board: &mut Board) {
        if board.is_legal_move(self.to_set.0, self.to_set.1, self.player) {
            self.to_remove = board.return_captured(self.to_set.0, self.to_set.1, self.player);
            for &(x, y) in &self.to_remove {
                board[(x, y)] = UNPLAYED_STATE;
                board.captures[(self.player - 1) as usize] += 1;
            }
            board[(self.to_set.0, self.to_set.1)] = self.player;
            self.set = true;
        }
    }

    pub fn unset(&mut self, board: &mut Board) {
        if self.set {
            for &(x, y) in &self.to_remove {
                board[(x, y)] = board.get_opponent(self.player);
                board.captures[(self.player - 1) as usize] -= 1;
            }
            board[(self.to_set.0, self.to_set.1)] = UNPLAYED_STATE;
            self.set = false;
        }
    }
}


// Main Board Structure for Game UI && MiniMax.
// Uses a Flat Vec<i32> and Index trait for easy indexing.
// Implements all game rules and utility functions.

#[derive(Clone, Data)]
pub struct Board {
    pub size : i32,
    #[data(eq)]
    pub board: Vec<i32>,
    #[data(eq)]
    pub captures : Vec<i32>,
}


impl Index<(i32, i32)> for Board {
    type Output = i32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.board[(index.0 * self.size + index.1) as usize]
    }
}

impl IndexMut<(i32, i32)> for Board {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        &mut self.board[(index.0 * self.size + index.1) as usize]
    }
}



impl Board {
    pub fn new(size: i32) -> Board {
        Board {
            size : size,
            board : vec![0; (size * size) as usize],
            captures : vec![0,0],
        }
    }

    pub fn is_legal_move(&mut self, x: i32, y: i32, player : i32) -> bool {
        if !self.is_valid(x, y) {
            return false;
        }
        if self[(x, y)] != UNPLAYED_STATE {
            return false;
        }
        if self.is_illegal_capture(x, y, player) {
            return false;
        }
        if self.is_double_three(x, y, player) {
            return false;
        }
        return true; 
    }

    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.size {
            return false;
        }
        if y < 0 || y >= self.size {
            return false;
        }
        return true;
    }


    pub fn return_captured(&self, x : i32, y : i32, player : i32) -> Vec<(i32, i32)> {
        let opp_player = self.get_opponent(player);

        for n in self.get_neighbours(x, y) {
            if self[(n.0, n.1)] == opp_player {
                let x_sym = x + ((n.0 - x) * 2);
                let y_sym = y + ((n.1 - y) * 2);
                let x_opp = x + ((n.0 - x) * 3);
                let y_opp = y + ((n.1 - y) * 3);
                if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
                    if self[(x_sym, y_sym)] == opp_player && self[(x_opp, y_opp)] == player {
                        return vec![(x_sym, y_sym), ((n.0, n.1))];
                    }
                }
            }
        }
        return vec![];
    }

    pub fn is_illegal_capture(&self, x: i32, y: i32, player: i32) -> bool {
        let opp_player = self.get_opponent(player);
        for n in self.get_neighbours(x, y) {
            if self[(n.0, n.1)] == player {
                let x_sym = x + ((n.0 - x) * 2);
                let y_sym = y + ((n.1 - y) * 2);
                let x_opp = x + ((n.0 - x) * -1);
                let y_opp = y + ((n.1 - y) * -1);
                if self.is_valid(x_sym, y_sym) && self.is_valid(x_opp, y_opp) {
                    if self[(x_sym, y_sym)] == opp_player && self[(x_opp, y_opp)] == opp_player {
                        return true;
                    }
                }
            }
        }
        return false;
    }


    /// .
    pub fn is_double_three(&mut self, x: i32, y: i32, player: i32) -> bool {
        let mut doubles = 0;
        
        self[(x, y)] = player;
        
        for i in 0..self.size {

            for j in 0..self.size {

                if self[(i, j)] == player {

                    for n in self.get_neighbours(i, j) {

                        if self[(n.0, n.1)] == player || self[(n.0, n.1)] == UNPLAYED_STATE {

                            let mut s_count = 1;
                            let mut b_count = 0;
                            let x2 = i + ((n.0 - i) * 2);
                            let y2 = j + ((n.1 - j) * 2);
                            let x3 = i + ((n.0 - i) * 3);
                            let y3 = j + ((n.1 - j) * 3);
                            let x0 = i + ((n.0 - i) * -1);
                            let y0 = j + ((n.1 - j) * -1);

                            if self.is_valid(x2, y2) && 
                                self.is_valid(x3, y3) && 
                                self.is_valid(x0, y0) {
                                
                                let p2 = self[(x2, y2)];
                                let p3 = self[(x3, y3)];
                                let p0 = self[(x0, y0)];
                                if p0 != UNPLAYED_STATE {
                                    continue;
                                }
                                if self[(n.0, n.1)] == UNPLAYED_STATE {
                                    b_count += 1;
                                }
                                else {
                                    s_count += 1;
                                }
                                if p2 == UNPLAYED_STATE {
                                    b_count += 1;
                                }
                                else if p2 == player {
                                    s_count += 1;
                                }
                                if p3 == UNPLAYED_STATE {
                                    b_count += 1;
                                }
                                else if p3 == player {
                                    s_count += 1;
                                }
                                if s_count == 3 && b_count == 1 {
                                    if p3 == UNPLAYED_STATE {
                                        doubles += 1;
                                    }
                                    else if p3 == player {
                                        let x4 = i + ((n.0 - i) * 4);
                                        let y4 = j + ((n.1 - j) * 4);
                                        if self.is_valid(x4, y4) {
                                            let p4 = self[(x4, y4)];
                                            if p4 == UNPLAYED_STATE {
                                                doubles += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self[(x, y)] = UNPLAYED_STATE;
        if doubles > 2 {
            return true;
        }
        return false;
    }

    pub fn game_over(&mut self, player : i32) -> bool {
        
        if self.return_winner() != UNPLAYED_STATE {
            return true;
        }
        if self.not_playable(player) {
            return true;
        }
        return false;
    }

    pub fn return_winner(&self) -> i32 {

        for x in 0..self.size {
            for y in 0..self.size {
                if self[(x, y)] != UNPLAYED_STATE {
                    if self.is_winner(x as i32, y as i32, self[(x, y)]) {
                        return self[(x, y)];
                    }
                }
            }
        }
        UNPLAYED_STATE
    }

    pub fn is_winner(&self, x : i32, y : i32, player : i32) -> bool {
        
        for n in self.get_neighbours(x, y) {
            if self[(n.0, n.1)] == player {
                let mut same_count = 2;
                for i in 2..self.size {
                    let x_sym = x + ((n.0 - x) * i);
                    let y_sym = y + ((n.1 - y) * i);
                    if self.is_valid(x_sym, y_sym) {
                        if self[(x_sym, y_sym)] != player {
                            break;
                        }
                        same_count += 1;
                    }
                }
                for i in 1..self.size {
                    let x_sym = x + ((n.0 - x) * -i);
                    let y_sym = y + ((n.1 - y) * -i);
                    if self.is_valid(x_sym, y_sym) {
                        if self[(x_sym, y_sym)] != player {
                            break;
                        }
                        same_count += 1;
                    }
                }
                if same_count > 4 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn not_playable(&mut self, player : i32) -> bool {
    
        for x in 0..self.size {
            for y in 0..self.size {
                if self.is_legal_move(x, y, player) {
                    return false;
                }
            }
        }
        return true;
    }
        
    pub fn get_neighbours(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        let mut neighbors = Vec::new();
        
        // Left
        if self.is_valid(x - 1, y) {
            neighbors.push((x - 1, y));
        }
        // Bottom Left
        if self.is_valid(x - 1, y - 1) {
            neighbors.push((x - 1, y - 1));
        }
        // Top Left
        if self.is_valid(x - 1, y + 1) {
            neighbors.push((x - 1, y + 1));
        }
        // Top
        if self.is_valid(x, y - 1) {
            neighbors.push((x, y - 1));
        }
        // Bottom Right
        if self.is_valid(x + 1, y - 1) {
            neighbors.push((x + 1, y - 1));
        }
        // Right
        if self.is_valid(x + 1, y) {
            neighbors.push((x + 1, y));
        }
        // Top Right
        if self.is_valid(x + 1, y + 1) {
            neighbors.push((x + 1, y + 1));
        }
        // Bottom
        if self.is_valid(x, y + 1) {
            neighbors.push((x, y + 1));
        }
        neighbors.into_iter()
    }

    pub fn get_opponent(&self, player: i32) -> i32 {
        if player == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE}    
    }

    pub fn count_alignements_of(of: i32, player: i32) -> i32 {
       0 
    }
}

