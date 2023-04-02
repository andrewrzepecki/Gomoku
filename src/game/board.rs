use crate::*;


// Implementation of a Board move to easily play and unplay moves.
//

pub struct BoardMove {
    pub x : i32,
    pub y : i32,
    pub player : i32,
    pub to_remove: Vec<(i32, i32)>,
    pub set : bool,
}

impl BoardMove {
    pub fn new(x: i32, y: i32, player: i32) -> BoardMove {
        BoardMove {
            x : x,
            y : y,
            player : player,
            to_remove : Vec::new(),
            set : false,
        }
    }

    pub fn set(&mut self, board: &mut Board) {
        if board.is_legal_move(self.x, self.y, self.player) {
            self.to_remove = board.return_captured(self.x, self.y, self.player);
            for &(x, y) in &self.to_remove {
                board[(x, y)] = UNPLAYED_STATE;
                board.captures[(self.player - 1) as usize] += 1;
            }
            board[(self.x, self.y)] = self.player;
            self.set = true;
        }
    }

    pub fn unset(&mut self, board: &mut Board) {
        if self.set {
            for &(x, y) in &self.to_remove {
                board[(x, y)] = board.get_opponent(self.player);
                board.captures[(self.player - 1) as usize] -= 1;
            }
            board[(self.x, self.y)] = UNPLAYED_STATE;
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


    // Main Logic & Rules for Gomoku.
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



    /// check if moves triggers an illegal double free three.
    pub fn is_double_three(&mut self, x: i32, y: i32, player: i32) -> bool {
        
        self[(x, y)] = player;
        let count = self.count_open_alignements_of(3, player);
        self[(x, y)] = UNPLAYED_STATE;
        return if count > 1 {true} else {false}
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

    pub fn get_delta(&self, p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        return (dx, dy);
    }

    pub fn count_open_alignements_of(&self, of: i32, player: i32) -> i32 {
        
        let mut alignements = 0;
        for x in 0..self.size {
            for y in 0..self.size {

                if self[(x, y)] == UNPLAYED_STATE {
                    for n in self.get_neighbours(x, y) {
                        
                        // Count on the line from current black + player number of players and blanks.
                        if self[(n.0, n.1)] == player {
                            let mut player_count = 1;
                            let mut blank_count = 0;
                            let (dx, dy) = self.get_delta((x, y), (n.0, n.1));
                            for i in 1..of {
                                let check_x = n.0 + (dx * i);
                                let check_y = n.1 + (dy * i);
                                if self.is_valid(check_x, check_y) {
                                    if self[(check_x, check_y)] == player {
                                        player_count += 1;
                                    }
                                    else {
                                        blank_count += 1;
                                    }
                                }
                            }
                            // If all were player, check for following blank.
                            if player_count == of {
                                let final_x = n.0 + (dx * of);
                                let final_y = n.1 + (dy * of);
                                if self.is_valid(final_x, final_y) {
                                    if self[(final_x, final_y)] == UNPLAYED_STATE {
                                        alignements += 1;
                                    }
                                }
                            }
                            // If has one blank and all remaining player, check for next two to be player and blank.
                            else if player_count == (of - 1) && blank_count == 1 {
                                let final_x = n.0 + (dx * of);
                                let final_y = n.1 + (dy * of);
                                if self.is_valid(final_x, final_y) {
                                    if self[(final_x, final_y)] == player {
                                        let blank_x = n.0 + (dx * (of + 1));
                                        let blank_y = n.1 + (dy * (of + 1));
                                        if self.is_valid(blank_x, blank_y) {
                                            if self[(final_x, final_y)] == UNPLAYED_STATE {
                                                alignements += 1;
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

        // All alignements will be counted twice, so return half.
        return alignements / 2 as i32;
    }
}

