use druid::widget::prelude::*;
use druid::{Size, EventCtx, Data, Color, Point};
use crate::GameData::AppState;
use druid::kurbo::{self, Circle};

#[derive(Clone, Data)]
pub struct BoardPiece {
    x : i32,
    y : i32,
    position : Point,
    radius: f64,
    state : i32,
}


impl BoardPiece {
    pub fn new(x: i32, y:i32, position : Point, radius: f64, state : i32) -> BoardPiece {
        BoardPiece {
            x,
            y,
            position,
            radius,
            state,
        }
    }
}

impl Widget<AppState> for BoardPiece {
    fn event(&mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env
    ) {
        if let Event::MouseDown(event) = event {
            if (self.position - event.pos).hypot() <= self.radius {
                // Unplayed Check
                if self.state == 0 {
                    self.state = data.turn;
                    data.board[self.x as usize][self.y as usize] = self.state;
                    data.turn = if data.turn == 1 {2} else {1};
                    println!("{:?}", data.board);
                }
            }
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
    }

    fn update(&mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        bc.max()
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        //let radius = size.width.min(size.height) / 2.0;
        // Set the child's size to the button size so it's properly centered
        //self.button.set_origin(ctx, data, env, size.to_vec2().0);
        //self.button.set_size(ctx, data, env, size);
        // Return the size of the circle to ensure the button is properly sized in its container
        //Size::new(radius * 2.0, radius * 2.0)
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, 
        ctx: &mut PaintCtx,
        data: &AppState,
        env: &Env
    ) {
        let x_delta = ctx.size().width / data.board_size as f64;
        let y_delta = ctx.size().height / data.board_size as f64;
        let x = x_delta * (self.x as f64) + (x_delta / 2.0);
        let y = y_delta * (self.y as f64) + (y_delta / 2.0);
        self.radius = (x_delta + y_delta) / 2.0 / 4.0;
        self.position = Point::new(x, y);
        let mut color = Color::TRANSPARENT;
        if self.state != 0 {
            color = if self.state == 1 {Color::BLACK} else {Color::WHITE};
        }
        ctx.fill(Circle::new(self.position, self.radius), &color);
    }
}
