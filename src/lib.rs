pub mod game;
pub mod data;
pub mod views;
pub mod minimax;

// Gomoku imports for main
pub use data::{
    app_state::*,
    game_mode::*,
    game_state::*,
    players::*
};

pub use game::{
    goban::*, 
    board_piece::*, 
    board::*,
    board_move::*,
    board_utils::*,
    cursor::*
};

pub use views::{
    main_view::*,
    menu_view::*,
    game_view::*,
    end_view::*, 
};

pub use minimax::{
 get_best_move::*,
 negamax::*,
 heuristics::*,
 // mtdf::*
};

// Druid 0.8.3 & external crate imports
use core::fmt;
pub use druid::{AppLauncher, LocalizedString, WindowDesc, Color, Cursor, im::Vector, 
                kurbo::{Line, Circle}, Point, Size, Data, EventCtx, Lens, Insets};
pub use druid::{Widget, WidgetExt, WidgetPod,
                widget::{Flex, Label, Align, Button, Controller, ViewSwitcher}};
pub use std::time::{Instant, Duration};
pub use druid::widget::prelude::*;
use std::collections::HashMap;

// Globals for default values.
pub const BOARDSIZE : usize = 19;
pub const MAX_CAPTURES : i32 = 10;

// Algorithm HyperParameters.
pub const DEPTH : usize = 2;
pub const CANDIDATE_SELECT : usize = 3;
pub const OPPONENT_WEIGHT : f64 = 1.00;
pub const DEFENSE_WEIGHT : f64 = 1.10;

// Bitboard pattern numbers
pub const DOUBLE_THREE : u64 = 84;
pub const FIVE_IN_A_ROW : u64 = 341;
