use druid::{AppLauncher, Widget, LocalizedString, WindowDesc, im::Vector, widget::{Button, Flex, Label, Align}, Size, Point, WidgetExt};
mod GameData;
mod BoardPiece;
mod Board;
use GameData::AppState;
use crate::BoardPiece::BoardPiece as OBoardPiece;
use crate::Board::Board as OBoard;
/*
*Data
*UI Builder
*Main
*/

const BOARDSIZE : i32 = 15;
const UNPLAYED_STATE : i32 = 0;
const PLAYER1_STATE : i32 = 1;
const PLAYER2_STATE : i32 = 2;

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
            let mut point = Point::new(0.0, 0.0);
            let mut radius = 40.0;
            let piece: OBoardPiece = OBoardPiece::new(
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
        .with_flex_child(Align::centered(OBoard::new(pieces)), 1.0);

    col
}