use crate::*;


pub fn main_view() -> impl Widget<AppState> {
    let mut current_view = Flex::row();
    current_view.add_child(
        Label::new(|data: &i32, _env: &Env| format!("Current view: {data}"))
            .lens(AppState::current_view),
    );
    //for i in 0..6 {
    //    switcher_column.add_spacer(80.);
    //    switcher_column.add_child(
    //        Button::new(format!("View {i}"))
    //            .on_click(move |_event, data: &mut u32, _env| {
    //                *data = i;
    //            })
    //            .lens(AppState::current_view),
    //    );
    //}

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, data, _env| match selector {
            0 => Box::new(menu_view()),
            1 => Box::new(game_view(data.board_size)),
            2 => Box::new(end_view()),
            //3 => Box::new(
            //    Flex::column()
            //        .with_flex_child(Label::new("Here is a label").center(), 1.0)
            //        .with_flex_child(
            //            Button::new("Button").on_click(|_event, _data, _env| {
            //                println!("Complex button clicked!");
            //            }),
            //            1.0,
            //        )
            //        .with_flex_child(TextBox::new().lens(AppState::current_text), 1.0)
            //        .with_flex_child(
            //            Label::new(|data: &String, _env: &Env| format!("Value entered: {data}"))
            //                .lens(AppState::current_text),
            //            1.0,
            //        ),
            //),
            _ => Box::new(Label::new("Unknown").center()),
        },
    );

    Flex::column()
        .with_child(current_view)
        .with_flex_child(view_switcher, 1.0)
}