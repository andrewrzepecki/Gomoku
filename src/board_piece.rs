use druid::widget::prelude::*;
use druid::{Size, EventCtx, Data, Color, Point};
use crate::{UNPLAYED_STATE, PLAYER1_STATE, PLAYER2_STATE};
use crate::game_data::AppState;
use crate::game_rules::{is_legal, check_capture, is_winner};
use druid::kurbo::Circle;

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
        _ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        _env: &Env
    ) {
        if let Event::MouseDown(event) = event {
            if (self.position - event.pos).hypot() <= self.radius {
                // Unplayed Check
                if is_legal(&data.board, self.x, self.y, data.turn) {
                    if check_capture(&mut data.board, self.x, self.y, data.turn) {
                        data.captures[(data.turn - 1) as usize] += 2;
                    }
                    if is_winner(&mut data.board, self.x, self.y, data.turn) {
                        data.winner = data.turn;
                    }
                    data.board[self.x as usize][self.y as usize] = data.turn;
                    data.turn = if data.turn == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
                }
            }
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &AppState,
        data: &AppState,
        _env: &Env,
    ) {
        self.state = data.board[self.x as usize][self.y as usize];
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        
        bc.max()
    } 

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, 
        ctx: &mut PaintCtx,
        data: &AppState,
        _env: &Env
    ) {
        let x_delta = ctx.size().width / data.board_size as f64;
        let y_delta = ctx.size().height / data.board_size as f64;
        let x = x_delta * (self.x as f64) + (x_delta / 2.0);
        let y = y_delta * (self.y as f64) + (y_delta / 2.0);
        self.radius = (x_delta + y_delta) / 2.0 / 4.0;
        self.position = Point::new(x, y);
        let mut color = Color::TRANSPARENT;
        if self.state != UNPLAYED_STATE {
            color = if self.state == PLAYER1_STATE {Color::BLACK} else {Color::WHITE};
        }
        ctx.fill(Circle::new(self.position, self.radius), &color);
    }
}
