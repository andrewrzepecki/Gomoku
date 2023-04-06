


#[cfg(test)]
pub mod tests {

    extern crate gomoku;

    use gomoku::*;

    fn build_test(pieces: Vector<BoardPiece>) -> impl Widget<AppState> {


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
            
            
        let player_one_button = Flex::row()
            .with_flex_child(Button::new("PLAYER ONE")
                .on_click(|_, data: &mut AppState, _| {
                    data.turn = PLAYER1_STATE;
                }), 1.0)
                .fix_width(220.0);
            
        let player_two_button = Flex::row()
            .with_flex_child(Button::new("PLAYER TWO")
                .on_click(|_, data: &mut AppState, _| {
                    data.turn = PLAYER2_STATE;
                }), 1.0)
                .fix_width(220.0);

        let reset_button = Flex::row()
            .with_flex_child(Button::new("RESET")
                .on_click(|_, data: &mut AppState, _| {
                   let _ = data.reset();
                }), 1.0)
                .fix_width(220.0);
        
        let button_row = Flex::row()
                .with_flex_child(player_one_button.center(), 1.0)
                .with_flex_child(player_two_button.center(), 1.0)
                .with_flex_child(reset_button.center(), 1.0);

        let col = Flex::column()
            .with_flex_child(Label::new("Gomoku - Testing").with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
            .with_flex_child(Label::new(|data: &AppState, _env: &Env| {format!("Score for player ONE: {}", data.player1_score)}).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
            .with_flex_child(Label::new(|data: &AppState, _env: &Env| {format!("Score for player TWO: {}", data.player2_score)}).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
            .with_flex_child(Align::centered(Goban::new(pieces)), 10.0)
            .with_flex_child(game_data_col, 2.0)
            .with_flex_child(button_row, 1.0);
        col

    }



    #[test]
    pub fn test_score() {
        
        let mut initial_state = AppState::default();

        let pieces = initial_state.test();

        let window = WindowDesc::new(build_test(pieces))
            .title(LocalizedString::new("Gomoku Settings"));

        let launcher = AppLauncher::with_window(window);
            launcher
            .launch(initial_state)
            .expect("launch failed");
    }
} 