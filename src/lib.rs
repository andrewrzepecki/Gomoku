pub mod game;
pub mod data;
pub mod views;
//pub mod minimax;

// Gomoku imports for main
pub use data::app_state::*;
pub use game::{
    board::*,
    board_utils::*
};
pub use views::{
    main_view::*,
    menu_view::*,
    game_view::*,
    end_view::*, 
    goban::*, 
    board_piece::*, 
    cursor::*
};
//pub use minimax::{get_move::*, negamax::*, heuristics::*, mtdf::*};

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
pub const DOUBLE_THREE : u64 = 84;
pub const FIVE_IN_A_ROW : u64 = 341;
