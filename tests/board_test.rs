#[cfg(test)]
pub mod score_tests {

    extern crate gomoku;

    use gomoku::*;

    pub fn test_view() -> impl Widget<AppState> {
        let game_data_col = Flex::column()
            .with_flex_child(Label::new(
                |data: &AppState, _env: &Env| {format!("Current Player: {}", data.color_names[if data.turn == Players::PlayerOne {data.player_colors[0] as usize} else {data.player_colors[1] as usize}])}
            ).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)

            .with_flex_child(Label::new(
                |data: &AppState, _env: &Env| {format!("Candidate Score: {}", data.candidate_score)}
            ).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)

            .with_flex_child( Button::new("P1")
                .on_click(
                    |_ctx, data: &mut AppState, _| {
                        // Reset AppState & launch new window.
                        data.turn = Players::PlayerOne;
                    }
                ), 1.0)
            .with_flex_child( Button::new("P2")
                  .on_click(
                      |_ctx, data: &mut AppState, _| {
                          // Reset AppState & launch new window.
                          data.turn = Players::PlayerTwo;
                      }
                  ), 1.0)
        .with_flex_child( Button::new("Reset")
                  .on_click(
                      |_ctx, data: &mut AppState, _| {
                          // Reset AppState & launch new window.
                          data.reset();
                      }
                  ), 1.0);
        let col = Flex::column()
            .with_flex_child(Label::new("Gomoku").with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
            .with_flex_child(Align::centered(Goban::new().controller(CursorArea {})), 10.0)
            .with_flex_child(game_data_col, 2.0);
        col
    }

    #[test]
    pub fn score_tests() {
        let mut initial_state = AppState::default();
        initial_state.game_mode = GameMode::PvP;
        initial_state.is_test = true;

        let window = WindowDesc::new(test_view())
            .title(LocalizedString::new("Gomoku Score Testing"))
            .window_size(Size::new(600.0, 450.0));

        AppLauncher::with_window(window)
            .log_to_console()
            .launch(initial_state)
            .expect("launch failed");
    }
}


#[cfg(test)]
pub mod board_tests {

    extern crate gomoku;

    use gomoku::*;


    #[test]
    pub fn test_board_get_and_set() {
        
        let mut board = Board::new();
        
        let mut first = board.get_state(0,0);
        let mut last = board.get_state(18, 18);
        assert_eq!(last, Players::Unplayed);
        assert_eq!(first, Players::Unplayed);
        board.print();

        board.set_state(0, 0, Players::PlayerOne);
        first = board.get_state(0, 0);
        assert_eq!(first, Players::PlayerOne);
        board.print();
        
        board.set_state(18, 18, Players::PlayerTwo);
        last = board.get_state(18, 18);
        assert_eq!(last, Players::PlayerTwo);
        board.print();

        board.set_state(0, 0, Players::Unplayed);
        first = board.get_state(0, 0);
        assert_eq!(first, Players::Unplayed);
        board.print();
        
        for _ in 0..1000000 {
            board.set_state(8, 8, Players::PlayerTwo);
            first = board.get_state(8, 8);
            let _ = board.move_is_legal(0, 0, Players::PlayerOne);
        }
        assert_eq!(first, Players::PlayerTwo);
        board.print();
    }

    #[test]
    pub fn test_scanning() {
        let mut board = Board::new();

        board.set_state(1, 0, Players::PlayerOne);
        board.set_state(2, 0, Players::PlayerOne);
        board.set_state(3, 0, Players::PlayerOne);
        let pattern = 21u64;
        let len = 4usize;

        let found = board.scan_position(1, 0, pattern, len, true);
        assert_eq!(found, 1);
        board.print();
    }

} 