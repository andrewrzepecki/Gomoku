pub mod game;
pub mod data;
pub mod minimax;

// Gomoku imports for main
pub use data::AppState;
pub use game::{board::*, board_piece::*, utils::*, rules::*, builder::*};
pub use minimax::{negamax::*, heuristics::*};

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
pub const DEPTH : i32 = 1;
pub const MAX_CAPTURES : i32 = 5;
