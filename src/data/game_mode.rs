use crate::*;

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