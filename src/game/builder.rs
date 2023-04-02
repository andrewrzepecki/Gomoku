use crate::*; 

pub fn build_menu() -> impl Widget<AppState> {
    
    // Create Board Size Picker
    let board_size_label = Label::new(format!("{:width$}","Board Size", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let board_size_value = Label::new(|data: &AppState, _env: &Env| data.board_size.to_string()).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let board_size_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size + 1).clamp(15, 30) });
    let board_size_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size - 1).clamp(15, 30) });
    let board_size_row = Flex::row()
        .with_flex_child(board_size_label, 1.0)
        .with_flex_child(board_size_down_button, 1.0)
        .with_flex_child(board_size_value, 1.0)
        .with_flex_child(board_size_up_button, 1.0);

    // Create Game Mode picker.
    let game_mode_label = Label::new(format!("{:width$}","Game Mode", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let game_mode_value = Label::new(|data: &AppState, _env: &Env|{format!("{}", data.game_mode)}).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let game_mode_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env| if data.game_mode == "PvP" {data.game_mode = "PvAI".into();} else if data.game_mode == "PvAI" {data.game_mode = "AIvAI".into();} else {data.game_mode = "PvP".into();});
    let game_mode_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env| if data.game_mode == "PvP" {data.game_mode = "AIvAI".into();} else if data.game_mode == "PvAI" {data.game_mode = "PvP".into();} else {data.game_mode = "PvAI".into();});
    let game_mode_row = Flex::row()
        .with_flex_child(game_mode_label, 1.0)
        .with_flex_child(game_mode_down_button, 1.0)
        .with_flex_child(game_mode_value, 1.0)
        .with_flex_child(game_mode_up_button, 1.0);

    // Create Player 1 color picker.
    let player1_color_label: Label<AppState> = Label::new(format!("{:width$}","Player 1 Color", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player1_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player1_color as usize].to_string()).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player1_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color + 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player1_color = (data.player1_color - 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_row = Flex::row()
        .with_flex_child(player1_color_label, 1.0)
        .with_flex_child(player1_color_down_button, 1.0)
        .with_flex_child(player1_color_value, 1.0)
        .with_flex_child(player1_color_up_button, 1.0);

    // Create Player 2 color picker.
    let player2_color_label: Label<AppState> = Label::new(format!("{:width$}","Player 2 Color", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player2_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player2_color as usize].to_string()).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player2_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color + 1).clamp(0, (data.colors.len() - 1)as i32) });
    let player2_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player2_color = (data.player2_color - 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player2_color_row = Flex::row()
        .with_flex_child(player2_color_label, 1.0)
        .with_flex_child(player2_color_down_button, 1.0)
        .with_flex_child(player2_color_value, 1.0)
        .with_flex_child(player2_color_up_button, 1.0);
        
    // Create the play button
    let play_button = Flex::row()
        .with_flex_child(Button::new("PLAY")
            .on_click(|ctx, data: &mut AppState, _| {
                let pieces = data.reset();
                ctx.window().close();
                let game_window = WindowDesc::new(build_game(pieces)).title(LocalizedString::new("Gomoku"));
                ctx.new_window(game_window);

            }), 1.0)
            .fix_width(220.0);

    // Create the vertical layout
    let layout = Flex::column()
        .with_flex_spacer(2.0)
        .with_flex_child(board_size_row, 1.0)
        .with_flex_child(game_mode_row, 1.0)
        .with_flex_child(player1_color_row, 1.0)
        .with_flex_child(player2_color_row, 1.0)
        .with_flex_child(play_button, 1.2)
        .padding(Insets::new(200.0, 0.0, 0.0, 0.0));
    layout

}


fn build_game(pieces: Vector<BoardPiece>) -> impl Widget<AppState> {

    let game_data_col = Flex::column()
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Current Player: {}", data.turn)}
        ).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player one captures: {}", data.captures[0])}
        ).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Player two captures: {}", data.captures[1])}
        )
        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
        .with_flex_child(Label::new(|data: &AppState, _: &Env| {
                format_duration(data.last_move_duration)
            })
            .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
            .border(Color::BLACK, 1.0)
            .center(), 1.0);

    let col = Flex::column()
        .with_flex_child(Label::new("Gomoku").with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
        .with_flex_child(Align::centered(Goban::new(pieces)), 10.0)
        .with_flex_child(game_data_col, 2.0);
    col
}


fn format_duration(duration: Duration) -> String {
    
    let millis = duration.as_millis() % 1000;
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 3600;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

pub fn build_winner() -> impl Widget<AppState> {
    
    let winner_label = Label::new(|data: &AppState, _env: &Env| {format!("Player {} Wins!", data.winner)})
        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    
    let play_again_button = Button::new("Play Again")
        .on_click(
            |ctx, data: &mut AppState, _| {
                // Reset AppState & launch new window.
                let pieces = data.reset();
                ctx.window().close();
                let game_window = WindowDesc::new(build_game(pieces)).title(LocalizedString::new("Gomoku"));
                data.winner_opened = true;
                ctx.new_window(game_window);
            }
        );

        let settings_button = Button::new("Settings")
            .on_click(
            |ctx, _: &mut AppState, _| {
                // Reset AppState & launch new window.
                ctx.window().close();
                let settings_window = WindowDesc::new(build_menu())
                    .title(LocalizedString::new("Gomoku Settings"))
                    .resizable(false)
                    .window_size(Size::new(600.0, 450.0)
                );
                ctx.new_window(settings_window);
            }
        );
        let exit_button = Button::new("Exit")
            .on_click(
            |ctx, _: &mut AppState, _| {
                ctx.window().close();
            }
        );
    
    
    let col = Flex::column()
        .with_flex_spacer(3.0)
        .with_flex_child(winner_label, 1.5)
        .with_flex_child(play_again_button, 1.0)
        .with_flex_child(settings_button, 1.0)
        .with_flex_child(exit_button, 1.0);
    col
}

pub fn build_pieces(size : i32) -> Vector<BoardPiece> {
    
    let mut pieces = Vector::new();
    
    for x in 0..size {
        for y in 0..size {
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
    pieces
}