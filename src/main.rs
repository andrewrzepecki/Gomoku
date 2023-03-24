mod board;
mod board_piece;
mod game_data;
mod game_rules;
mod utils;

use druid::WidgetExt;
use druid::piet::PaintBrush;
use game_data::AppState;
use board_piece::BoardPiece;
use board::Board;
use druid::widget::{prelude::*, BackgroundBrush};
use druid::{AppLauncher, Widget, LocalizedString, WindowDesc, im::Vector, widget::{Flex, Label, Align, Padding, Button}, Point, Color};


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
    let window = WindowDesc::new(build_menu()).title(LocalizedString::new("Gomoku"));
    let initial_state = AppState {
        label : "Test Game".into(),
        board_size : BOARDSIZE,
        turn : 1,
        player1_color : 0,
        player2_color : 1,
        board : board,
        captures: Vec::from([0,0]),
        winner : 0,
        game_mode : "PvP".into(),
        colors : Vec::from([Color::BLACK, Color::WHITE, Color::BLUE, Color::RED, Color::GREEN, Color::YELLOW, Color::SILVER]),
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}

fn build_game() -> impl Widget<AppState> {

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
    
    let game_data_col = Flex::column()
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Current Player: {}", data.turn)}
        ), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player one captures: {}", data.captures[0])}
        ), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player two captures: {}", data.captures[1])}
        ), 1.0);


    let col = Flex::column()
        .with_flex_child(Label::new("Gomoku"), 0.2)
        .with_flex_child(Align::centered(Board::new(pieces)), 1.0)
        .with_flex_child(game_data_col, 0.2);
    col
}




fn build_menu() -> impl Widget<AppState> {
    
    
    let board_size_label = Label::new("Board Size:");
    let board_size_value = Label::new(|data: &AppState, _env: &Env| data.board_size.to_string());
    let board_size_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size + 1).clamp(15, 30) });
    let board_size_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size - 1).clamp(15, 30) });
    let board_size_row = Flex::row()
        .with_child(board_size_label)
        .with_flex_child(
            Flex::row()
                .with_child(board_size_down_button)
                .with_child(Padding::new(5.0, board_size_value))
                .with_child(board_size_up_button),
            1.0,
        );



    let game_mode_label = Label::new("Game Mode:");
    let game_mode_value = Label::new(|data: &AppState, _env: &Env|{format!("{}", data.game_mode)});
    let game_mode_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env| if data.game_mode == "PvP" {data.game_mode = "PvAI".into();} else if data.game_mode == "PvAI" {data.game_mode = "AIvAI".into();} else {data.game_mode = "PvP".into();});
    let game_mode_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env| if data.game_mode == "PvP" {data.game_mode = "AIvAI".into();} else if data.game_mode == "PvAI" {data.game_mode = "PvP".into();} else {data.game_mode = "PvAI".into();});
    let game_mode_row = Flex::row()
        .with_flex_child(game_mode_label, 1.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(game_mode_down_button, 1.0)
                .with_flex_child(Padding::new(5.0, game_mode_value), 1.0)
                .with_flex_child(game_mode_up_button, 1.0),
            1.0,
        );




    let player1_color_label: Label<AppState> = Label::new("Player 1 Color");
    let player1_color_value = Label::new(|data: &AppState, _env: &Env| data.player1_color.to_string());
    let player1_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color + 1).clamp(0, data.colors.len() as i32) });
    let player1_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color - 1).clamp(0, data.colors.len() as i32) });
    let player1_color_row = Flex::row()
        .with_child(player1_color_label)
        .with_flex_child(
            Flex::row()
                .with_child(player1_color_down_button)
                .with_child(Padding::new(5.0, player1_color_value))
                .with_child(player1_color_up_button),
            1.0,
        );

    let player2_color_label: Label<AppState> = Label::new("Player 2 Color");
    let color = |data: &AppState| {data.colors[data.player2_color as usize]};
    let player2_background = BackgroundBrush::from(BackgroundBrush::Color(Color::WHITE));
    let player2_color_value = Label::new(|data: &AppState, _env: &Env| data.player2_color.to_string()).background(player2_background);
    let player2_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color + 1).clamp(0, data.colors.len() as i32) });
    let player2_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color - 1).clamp(0, data.colors.len() as i32) });
    let player2_color_row = Flex::row()
        .with_flex_child(player2_color_label, 1.0)
        .with_flex_child(
            Flex::row()
                .with_child(player2_color_down_button)
                .with_child(Padding::new(5.0, player2_color_value))
                .with_child(player2_color_up_button),
            1.0,
        );

    
    
    // Create the play button
    let play_button = Button::new("PLAY")
        .on_click(|_, data: &mut AppState, _| {
            // Do something with the settings
            println!("{}", data.player1_color);
            // TODO: Close the settings window and open the game window
        });

    // Create the vertical layout
    let layout = Align::centered(Flex::column()
        .with_child(board_size_row)
        .with_child(game_mode_row)
        .with_spacer(20.0)
        .with_child(player1_color_row)
        .with_child(player2_color_row)
        .with_spacer(20.0)
        .with_child(play_button));
    layout

}