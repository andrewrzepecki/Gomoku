mod board;
mod board_piece;
mod game_data;
mod game_rules;
mod utils;

use game_data::AppState;
use board_piece::BoardPiece;
use board::Board;
use druid::{AppLauncher, Widget, LocalizedString, WindowDesc, im::Vector, widget::{Flex, Label, Align}, Point};


const BOARDSIZE : i32 = 19;
pub const UNPLAYED_STATE : i32 = 0;
pub const PLAYER1_STATE : i32 = 1;
pub const PLAYER2_STATE : i32 = 2;

pub fn main() {
    let mut board = Vec::new();
    for _ in  0..BOARDSIZE {
        let mut row = Vec::new();
        for _ in 0..BOARDSIZE {
            row.push(0);
        }
        board.push(row);
    }
    let window = WindowDesc::new(build_widget()).title(LocalizedString::new("Gomoku"));
    let initial_state = AppState {
        label : "Test Game".into(),
        board_size : BOARDSIZE,
        turn : 1,
        board : board,
        captures : vec![0,0],
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}

fn build_widget() -> impl Widget<AppState> {

    let mut pieces = Vector::new();
    for x in 0..BOARDSIZE {
        for y in 0..BOARDSIZE {
            let point = Point::new(0.0, 0.0);
            let radius = 40.0;
            let piece: BoardPiece = BoardPiece::new(
                x,
                y,
                point,
                radius,
                0,
            );
            pieces.push_back(piece);
        }
    }
    let col = Flex::column()
        .with_flex_child(Label::new("Gomoku"), 0.2)
        .with_flex_child(Align::centered(Board::new(pieces)), 1.0);

    col
}