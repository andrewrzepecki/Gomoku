pub mod board;
pub mod board_piece;
pub mod game_data;
pub mod game_rules;
pub mod utils;
pub mod builder;
pub mod negamax;

// Gomoku imports for main
pub use game_data::AppState;
pub use board::Board;
pub use board_piece::BoardPiece;
pub use builder::build_menu;

// Druid 0.8.3 imports
pub use druid::{AppLauncher, LocalizedString, WindowDesc, Color};

// Globals for default values.
pub const BOARDSIZE : i32 = 19;
pub const UNPLAYED_STATE : i32 = 0;
pub const PLAYER1_STATE : i32 = 1;
pub const PLAYER2_STATE : i32 = 2;
