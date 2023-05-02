
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum  Players {
    PlayerOne,
    PlayerTwo,
    Unplayed,
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
