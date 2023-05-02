

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum  GameState {
    Menu,
    Game,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}