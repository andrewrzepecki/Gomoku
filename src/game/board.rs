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
        if !self.set {
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
    #[data(eq)]
    pub score_table : HashMap<String, (i32, bool)>,
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
            score_table : Board::get_score_table(),
        }
    }
    
    // Main Pattern Scoring interface.
    pub fn get_score_table() -> HashMap<String, (i32, bool)> {
        let mut map = HashMap::new();
        
        // --- Five in a Row
        map.insert("xxxxx".to_string(), (100000, false));
        
        // --- Live Four
        map.insert("0xxxx0".to_string(), (50000, false));
        
        // --- Dead Four
        map.insert("oxxxx0".to_string(), (5000, true));
        map.insert("0x0xxx0".to_string(), (5000, true));
        map.insert("0xx0xx0".to_string(), (5000, false));
        
        // --- Live Three
        map.insert("0xxx0".to_string(), (5000, false));
        
        // --- Dead Three
        map.insert("0xx0x0".to_string(), (100, true));
        map.insert("oxxx0".to_string(), (100, true));
        map.insert("oxx0x0".to_string(), (100, true));
        map.insert("ox0xx0".to_string(), (100, true));
        map.insert("0xx00x0".to_string(), (100, true));
        map.insert("0x0x0x0".to_string(), (100, false));
        map.insert("o0xxx0o".to_string(), (100, false));
        
        // --- Live Two
        map.insert("0x000x0".to_string(), (500, false));
        map.insert("0x00x0".to_string(), (500, false));
        map.insert("0x0x0".to_string(), (500, false));
        // --- Dead Two
        map.insert("oxx0".to_string(), (10, true));
        map.insert("ox0x0".to_string(), (10, true));
        map.insert("ox00x0".to_string(), (10, true));
        map.insert("0xx0".to_string(), (10, false));
        
        map
    }
    
    pub fn get_hash(&self) -> String {

        let separator = "";
        let board_str = self.board
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(separator);
        board_str
    }

    pub fn get_lines(&self) -> Vec<String> {
        
        // Main function for analyzing board:
        // Return columns, rows, diags and anti-diagonals
        let mut lines = Vec::new();

        
        for x in 0..self.size {
            let mut col = String::new();
            let mut row = String::new();
            let mut diag_top = String::new();
            let mut diag_bottom = String::new();
            let mut anti_diag_top = String::new();
            let mut anti_diag_bottom = String::new();
            for y in 0..self.size {
                col.push_str(self[(x, y)].to_string().as_str());
                row.push_str(self[(y, x)].to_string().as_str());
                
                if self.is_valid(x + y , y) {
                    diag_top.push_str(self[(x + y, y)].to_string().as_str());
                    if x != 0 {
                        diag_bottom.push_str(self[(y, x + y)].to_string().as_str())
                    }
                }
                if self.is_valid((self.size - 1) - (x + y), y) {
                    anti_diag_top.push_str(self[((self.size - 1) - (x + y), y)].to_string().as_str());
                    if x != 0 {
                        anti_diag_bottom.push_str(self[((self.size - 1) -  y, (y + x))].to_string().as_str())
                    }
                }

            }
            lines.push(col);
            lines.push(row);
            lines.push(diag_top);
            lines.push(anti_diag_top);
            if x != 0 {
                lines.push(diag_bottom);
                lines.push(anti_diag_bottom)
            }
        }
        return lines;
    }


    pub fn count_line_occurrences(&self, vec: &str, subvec: &str) -> i32 {
        if subvec.is_empty() {
            return 0;
        }
        let subvec_len = subvec.len();
        if subvec_len > vec.len() {
            return 0;
        }
        let windows = vec.as_bytes().windows(subvec_len);
        windows.filter(|window| window.eq(&subvec.as_bytes())).count() as i32
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
        let count = self.count_free_open_alignements_of(3, player)
         + self.count_free_closed_alignements_of(3, player);
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
        // Check Five
        for x in 0..self.size {
            for y in 0..self.size {
                if self[(x, y)] != UNPLAYED_STATE {
                    if self.is_winner(x as i32, y as i32, self[(x, y)]) {
                        return self[(x, y)];
                    }
                }
            }
        }
        // Check Captures
        let mut winner = UNPLAYED_STATE;
        if self.captures[0] >= MAX_CAPTURES {
            winner = PLAYER1_STATE
        }
        else if self.captures[1] >= MAX_CAPTURES {
            winner = PLAYER1_STATE
        }
        winner
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

    pub fn count_free_open_alignements_of(&self, of: i32, player: i32) -> i32 {
        
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
                            // If has one blank and all remaining player, check for next two to be player and blank.
                            if player_count == (of - 1) && blank_count == 1 {
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

    pub fn count_free_closed_alignements_of(&self, of: i32, player: i32) -> i32 {
        let pattern = format!(
            "{}{}{}",
            UNPLAYED_STATE,
            player.to_string().as_str().repeat(of as usize),
            UNPLAYED_STATE
        );
        let mut count = 0;
        let all_lines = self.get_lines(); 
        for lt in all_lines {
            count += self.count_line_occurrences(&lt, &pattern);
        }
        return count;
    }

    pub fn count_blocked_closed_alignements_of(&self, of: i32, player: i32) -> i32 {
        let pattern = format!(
            "{}{}{}",
            UNPLAYED_STATE,
            player.to_string().as_str().repeat(of as usize),
            self.get_opponent(player)
        );
        let anti_pattern = pattern.chars().rev().collect::<String>();
        let mut count = 0;
        let all_lines = self.get_lines(); 
        for lt in all_lines {
            count += self.count_line_occurrences(&lt, &pattern);
            count += self.count_line_occurrences(&lt, &anti_pattern);
        }
        return count;
    }


    pub fn count_blocked_open_alignements_of(&self, of: i32, player: i32) -> i32 {
        
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
                            // If has one blank and all remaining player, check for next two to be player and blank.
                            if player_count == (of - 1) && blank_count == 1 {
                                let final_x = n.0 + (dx * of);
                                let final_y = n.1 + (dy * of);
                                if self.is_valid(final_x, final_y) {
                                    if self[(final_x, final_y)] == self.get_opponent(player) {
                                        alignements += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return alignements;
    }
}

