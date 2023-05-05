use crate::*;


pub fn main_view() -> impl Widget<AppState> {
    let mut current_view = Flex::row();
    current_view.add_child(
        Label::new(|data: &i32, _env: &Env| format!("Current view: {data}"))
            .lens(AppState::current_view),
    );

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            0 => Box::new(menu_view()),
            1 => Box::new(game_view()),
            2 => Box::new(end_view()),
            _ => Box::new(Label::new("Unknown").center()),
        },
    );

    Flex::column()
        .with_child(current_view)
        .with_flex_child(view_switcher, 1.0)
}