use crate::*;

// Needs argument to define board size:
//    -> Breaks code pattern, would prefer to be able to give a closure to Goban::new()
//        -> TODO: implement and test Goban::dynamic()
pub fn game_view(board_size: usize) -> Flex<AppState> {


    let settings_button = Button::new("Settings")
            .on_click(
            |_ctx, data: &mut AppState, _| {
                // Reset AppState & launch new window.
                data.reset();
                data.game_state = GameState::Menu;
                data.current_view = data.game_state as i32;
            }
        );

    let game_data_col = Flex::column()

        .with_flex_child(Label::new(
            |data: &AppState, _env: &Env| {format!("Current Player: {}", data.color_names[if data.turn == Players::PlayerOne {data.player_colors[0] as usize} else {data.player_colors[1] as usize}])}
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
            .center(), 1.0)
        
        .with_flex_child(settings_button, 1.0);

    let col = Flex::column()
        .with_flex_child(Label::new("Gomoku").with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
        .with_flex_child(Align::centered(Goban::new(board_size)), 10.0)
        .with_flex_child(game_data_col, 2.0);
    col
}

pub fn format_duration(duration: Duration) -> String {
    
    let millis = duration.as_millis() % 1000;
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 3600;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}