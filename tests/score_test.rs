


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
            
            
        let score_button = Flex::row()
            .with_flex_child(Button::new("GET SCORE")
                .on_click(|_, data: &mut AppState, _| {
                    data.test_score = get_final_score(&mut data.board, data.turn);
                }), 1.0)
                .fix_width(220.0);    
            
        let player_one_button = Flex::row()
            .with_flex_child(Button::new("PLAYER  ONE")
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
        
        let button_row = Flex::row()
                .with_flex_child(score_button, 1.0)
                .with_flex_child(player_one_button, 1.0)
                .with_flex_child(player_two_button, 1.0);

        let col = Flex::column()
            .with_flex_child(Label::new("Gomoku - Testing").with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
            .with_flex_child(Label::new(|data: &AppState, _env: &Env| {format!("Score for player {}: {}", data.test_score, data.turn)}).with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE)), 1.0)
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
    #[test]
    pub fn test_heuristic() {
        let mut data = AppState::default();
        let mut turn = 0;
        let mut player = PLAYER1_STATE;
        let pieces = data.reset();
        let mut tt: std::collections::HashMap<String, (i32, i32, i32)> =  std::collections::HashMap::new();
        let mut turns = 0; 
        let mut flag = true;
        data.game_mode = "AIvAI".into();
        while flag {
            let (x, y, score) = alpha_beta_minimax(
                &mut data.board,
                data.turn,
                DEPTH,
                true,
                -10000000,
                10000000, 
                &mut tt
            );
            let mut m = BoardMove::new(x, y, player);
            if data.board.is_legal_move(x, y, player) {
                m.set(&mut data.board);
            }
            player = data.board.get_opponent(player);
            turn += 1;
            if data.board.game_over(player) {
                flag = false;
            }
        }
        println!("Winner is player {} after {} turns", data.board.return_winner(), turns);
    }
}