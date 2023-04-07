pub mod game;
pub mod data;
pub mod minimax;

// Gomoku imports for main
pub use data::AppState;
pub use game::{goban::*, board::*, board_piece::*, builder::*};
pub use minimax::{get_move::*, negamax::*, heuristics::*, mtdf::*};

// Druid 0.8.3 imports
pub use druid::{AppLauncher, LocalizedString, WindowDesc, Color, im::Vector, kurbo::Line, Point, Size, Data, EventCtx, Lens, Insets};
pub use std::time::{Instant, Duration};
use std::collections::HashMap;
use rand::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, BufReader};
use core::ops::{IndexMut, Index};
pub use druid::widget::prelude::*;
pub use druid::{Widget, widget::{Flex, Label, Align, Button}};
pub use druid::kurbo::Circle;
pub use druid::WidgetExt;

// Globals for default values.
pub const BOARDSIZE : i32 = 19;
pub const GAME_MODE : &str = "PvAI";
pub const MAX_CAPTURES : i32 = 10;
pub const UNPLAYED_STATE : i32 = 0;
pub const PLAYER1_STATE : i32 = 1;
pub const PLAYER2_STATE : i32 = 2;
pub const TT_PATH : &str = "./tt.json";

// Algorithm HyperParameters.
pub const DEPTH : i32 = 2;
pub const CANDIDATE_SELECT : usize = 3;
pub const WARMP_UP : usize = 8;
pub const OPPONENT_WEIGHT : f64 = 1.00;
pub const DEFENSE_WEIGHT : f64 = 1.10;