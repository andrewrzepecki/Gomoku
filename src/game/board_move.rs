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
            board.free_threes[self.player as usize] += board.is_free_three(self.x, self.y, self.player) as u64;
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
            board.free_threes[self.player as usize] -= board.is_free_three(self.x, self.y, self.player) as u64;
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