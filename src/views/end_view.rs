use crate::*;

pub fn end_view() -> Flex<AppState> {

    let winner_label = Label::new(|data: &AppState, _env: &Env| {format!("Player {} Wins!", (data.winner.unwrap() as usize).to_string())})
        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE));
    
    let play_again_button = Button::new("Play Again")
        .on_click(
            |_, data: &mut AppState, _| {
                // Reset AppState & change game_state
                data.reset();
                data.game_state = GameState::Game;
                data.current_view = data.game_state as i32;
            }
        );

        let settings_button = Button::new("Settings")
            .on_click(
            |_, data: &mut AppState, _| {
                // Reset AppState & launch new window.
                data.reset();
                data.game_state = GameState::Menu; 
                data.current_view = data.game_state as i32;
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

    let pannel = Flex::row()
        .with_flex_child(
            // bottom left
            col.center(), 1.0
        )
        .with_flex_child(
            // bottom right
            Goban::new(), 1.0
        );
    pannel
}