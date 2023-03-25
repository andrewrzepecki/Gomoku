use druid::im::Vector;
use druid::{WindowDesc, LocalizedString, Point, Color};
use druid::widget::prelude::*;
use druid::{Widget, widget::{Flex, Label, Align, Padding, Button}};
use crate::{AppState, BoardPiece, Board};
use druid::WidgetExt;


use std::time::{Duration, Instant};
pub fn build_menu() -> impl Widget<AppState> {
    
    // Create Board Size Picker
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


    // Create Game Mode picker.
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



    // Create Player 1 color picker.
    let player1_color_label: Label<AppState> = Label::new("Player 1 Color");
    let player1_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player1_color as usize].to_string());
    let player1_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color + 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color - 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_row = Flex::row()
        .with_child(player1_color_label)
        .with_flex_child(
            Flex::row()
                .with_child(player1_color_down_button)
                .with_child(Padding::new(5.0, player1_color_value))
                .with_child(player1_color_up_button),
            1.0,
        );


    // Create Player 2 color picker.
    let player2_color_label: Label<AppState> = Label::new("Player 2 Color");
    let player2_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player2_color as usize].to_string());
    let player2_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color + 1).clamp(0, (data.colors.len() - 1)as i32) });
    let player2_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color - 1).clamp(0, (data.colors.len() - 1) as i32) });
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
        .on_click(|ctx, data: &mut AppState, _| {
            let mut pieces = Vector::new();
            for x in 0..data.board_size {
                for y in 0..data.board_size {
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
            let mut board = Vec::new();
            for _ in  0..data.board_size {
                let mut row = Vec::new();
                for _ in 0..data.board_size {
                    row.push(0);
                }
                board.push(row);
            }
            data.board = board.clone();
            ctx.window().close();
            let game_window = WindowDesc::new(build_game(pieces)).title(LocalizedString::new("Gomoku"));
            ctx.new_window(game_window);

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


fn build_game(pieces: Vector<BoardPiece>) -> impl Widget<AppState> {

    let game_data_col = Flex::column()
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Current Player: {}", data.turn)}
        ), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player one captures: {}", data.captures[0])}
        ), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player two captures: {}", data.captures[1])}
        ), 1.0)
        .with_flex_child(Label::new(|data: &AppState, _: &Env| {
                let time = std::time::Instant::now().duration_since(data.last_move_time);
                format_duration(time)
            })
            .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
            .border(Color::BLACK, 1.0)
            .center(), 1.0);


    let col = Flex::column()
        .with_flex_child(Label::new("Gomoku"), 0.2)
        .with_flex_child(Align::centered(Board::new(pieces)), 1.0)
        .with_flex_child(game_data_col, 0.2);
    col
}


fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis() % 1000;
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 3600;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}