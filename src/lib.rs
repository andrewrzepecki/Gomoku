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
pub use builder::{build_menu, build_winner, build_pieces, build_board};
pub use game_rules::{is_legal, is_winner, check_capture};
pub use negamax::alpha_beta_negamax;
pub use utils::{get_neighbours, is_valid_coords};

// Druid 0.8.3 imports
pub use druid::{AppLauncher, LocalizedString, WindowDesc, Color, im::Vector, kurbo::Line, Point, Size, Data, EventCtx, Lens, Insets};
pub use std::time::{Instant, Duration};
pub use druid::widget::prelude::*;
pub use druid::{Widget, widget::{Flex, Label, Align, Button}};
pub use druid::kurbo::Circle;
pub use druid::WidgetExt;

// Globals for default values.
pub const BOARDSIZE : i32 = 19;
pub const UNPLAYED_STATE : i32 = 0;
pub const PLAYER1_STATE : i32 = 1;
pub const PLAYER2_STATE : i32 = 2;
