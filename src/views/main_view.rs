use crate::*;


pub fn main_view() -> impl Widget<AppState> {

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            0 => Box::new(menu_view()),
            1 => Box::new(game_view()),
            2 => Box::new(end_view()),
            _ => Box::new(Label::new("ERROR: Contact Support Team ~andrew").center()),
        },
    );

    Flex::column()
        .with_flex_child(view_switcher, 1.0)
}