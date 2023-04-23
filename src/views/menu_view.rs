use crate::*; 

pub fn menu_view() -> Flex<AppState> {
    
    let top_pannel = Flex::row()
        .with_flex_child(
            // top left
            Label::new("Gomoku")
                .center()
                .border(Color::GRAY, 4.0)
                .padding(10.0), 1.0
        )
        .with_flex_child(
            // top right
            Label::new("tbc")
                .center()
                .background(Color::GRAY)
                .padding(10.0), 1.0
        );

    let bottom_pannel = Flex::row()
        .with_flex_child(
            // bottom left
            menu_pannel().padding(Insets::new(200.0, 0.0, 0.0, 0.0)), 1.0
        )
        .with_flex_child(
            // bottom right
            Button::new("PLAY")
            .on_click(
                |_ctx, data: &mut AppState, _| {
                    data.board = Board::new(data.board_size);
                    data.turn = Players::PlayerOne;
                    data.change_cursor(data.turn);
                    data.game_state = GameState::Game;
                    data.current_view = data.game_state as i32;
                }
            ) 
            .center()
            .padding(10.0), 1.0
        );

    let result = Flex::column()
        .with_flex_child(
            top_pannel, 1.0
        )
        .with_flex_child(
            bottom_pannel, 1.0
        );
    result
}



fn menu_pannel() ->  Flex<AppState> {
    
    // Create Board Size Picker
    let board_size_label = Label::new(format!("{:width$}","Board Size", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let board_size_value = Label::new(|data: &AppState, _env: &Env| format!("{:width$}",data.board_size.to_string(), width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let board_size_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size + 1).clamp(8, 30) });
    let board_size_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.board_size = (data.board_size - 1).clamp(8, 30) });
    let board_size_row = Flex::row()
        .with_flex_child(board_size_label, 1.0)
        .with_flex_child(board_size_down_button, 1.0)
        .with_flex_child(board_size_value, 1.0)
        .with_flex_child(board_size_up_button, 1.0);

    // Create Game Mode picker.
    let game_mode_label = Label::new(format!("{:width$}","Game Mode", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let game_mode_value = Label::new(|data: &AppState, _env: &Env|{format!("{}", data.game_mode)}).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let game_mode_up_button = Button::new("+")
        .on_click(
            |_ctx, data: &mut AppState, _env| {
                data.game_mode = match data.game_mode {
                    GameMode::PvAI => GameMode::PvP,
                    GameMode::PvP => GameMode::AIvAI,
                    GameMode::AIvAI => GameMode::PvAI,
                }
            }
        );
    let game_mode_down_button = Button::new("-")
        .on_click(
            |_ctx, data: &mut AppState, _env| {
                data.game_mode = match data.game_mode {
                    GameMode::PvAI => GameMode::AIvAI,
                    GameMode::AIvAI => GameMode::PvP,
                    GameMode::PvP => GameMode::PvAI,
                }
            }
        );
    let game_mode_row = Flex::row()
        .with_flex_child(game_mode_label, 1.0)
        .with_flex_child(game_mode_down_button, 1.0)
        .with_flex_child(game_mode_value, 1.0)
        .with_flex_child(game_mode_up_button, 1.0);

    // Create Player 1 color picker.
    let player1_color_label: Label<AppState> = Label::new(format!("{:width$}","Player 1 Color", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player1_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player_colors[0] as usize].to_string()).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player1_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player_colors[0] = (data.player_colors[0] + 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player_colors[0] = (data.player_colors[0] - 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player1_color_row = Flex::row()
        .with_flex_child(player1_color_label, 1.0)
        .with_flex_child(player1_color_down_button, 1.0)
        .with_flex_child(player1_color_value, 1.0)
        .with_flex_child(player1_color_up_button, 1.0);

    // Create Player 2 color picker.
    let player2_color_label: Label<AppState> = Label::new(format!("{:width$}","Player 2 Color", width=20)).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player2_color_value = Label::new(|data: &AppState, _: &Env| data.color_names[data.player_colors[1] as usize].to_string()).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    let player2_color_up_button = Button::new("+")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player_colors[1] = (data.player_colors[1] + 1).clamp(0, (data.colors.len() - 1)as i32) });
    let player2_color_down_button = Button::new("-")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| { data.player_colors[1] = (data.player_colors[1] - 1).clamp(0, (data.colors.len() - 1) as i32) });
    let player2_color_row = Flex::row()
        .with_flex_child(player2_color_label, 1.0)
        .with_flex_child(player2_color_down_button, 1.0)
        .with_flex_child(player2_color_value, 1.0)
        .with_flex_child(player2_color_up_button, 1.0);
        
    

    // Create the vertical layout
    
    let layout = Flex::column()
        .with_flex_spacer(2.0)
        .with_flex_child(board_size_row, 1.0)
        .with_flex_child(game_mode_row, 1.0)
        .with_flex_child(player1_color_row, 1.0)
        .with_flex_child(player2_color_row, 1.0);
        //.padding(Insets::new(200.0, 0.0, 0.0, 0.0));
    layout

}