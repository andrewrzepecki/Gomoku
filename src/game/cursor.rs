use crate::*;


/// This Controller switches the current cursor based on the selection.
/// The crucial part of this code is actually making and initialising
/// the cursor. This happens here. Because we cannot make the cursor
/// before the window is open we have to do that on `WindowConnected`.
struct CursorArea;

impl<W: Widget<AppState>> Controller<AppState, W> for CursorArea {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        //if let Event::WindowConnected = event {
        //    data.custom = ctx.window().make_cursor(&data.custom_desc);
        //}
        child.event(ctx, event, data, env);
    }

    fn update(
        &mut self,

        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        if data.cursor != old_data.cursor {
            ctx.set_cursor(&data.cursor);
        }
        child.update(ctx, old_data, data, env);
    }
}