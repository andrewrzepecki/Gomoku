pub mod game;
pub mod data;
pub mod minimax;

// Gomoku imports for main
pub use data::AppState;
pub use game::{goban::*, board::*, board_piece::*, builder::*};
pub use minimax::{negamax::*, heuristics::*};

// Druid 0.8.3 imports
pub use druid::{AppLauncher, LocalizedString, WindowDesc, Color, im::Vector, kurbo::Line, Point, Size, Data, EventCtx, Lens, Insets};
pub use std::time::{Instant, Duration};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use core::ops::{IndexMut, Index};
pub use druid::widget::prelude::*;
pub use druid::{Widget, widget::{Flex, Label, Align, Button}};
pub use druid::kurbo::Circle;
pub use druid::WidgetExt;
// Globals for default values.
pub const BOARDSIZE : i32 = 19;
pub const UNPLAYED_STATE : i32 = 0;
pub const PLAYER1_STATE : i32 = 1;
pub const PLAYER2_STATE : i32 = 2;
pub const DEPTH : i32 = 2;
pub const MAX_CAPTURES : i32 = 10;
