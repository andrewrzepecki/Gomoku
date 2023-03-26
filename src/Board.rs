use std::time::Instant;

use druid::{LocalizedString, WindowDesc};
use druid::widget::prelude::*;
use druid::{im::Vector, kurbo::Line, Point, Size, Color};

use crate::game_rules::{is_winner, check_capture};
use crate::{PLAYER1_STATE, PLAYER2_STATE, UNPLAYED_STATE};
use crate::board_piece::BoardPiece;
use crate::builder::build_winner;
use crate::game_data::AppState;
use crate::negamax::alpha_beta_negamax;

pub struct Board {
    pieces : Vector<BoardPiece>,
}

 impl Board {
    pub fn new(pieces : Vector<BoardPiece>) -> Board {
        Board {
            pieces,
        }
    }
 }

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<AppState> for Board {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env : &Env) {
        if data.is_ai[(data.turn - 1) as usize] {
            let _move = alpha_beta_negamax(&mut data.board, data.turn, 10, 0, 0);
            if check_capture(&mut data.board, _move.0, _move.1, data.turn) {
                data.captures[(data.turn - 1) as usize] += 2;
            }
            if is_winner(&mut data.board, _move.0, _move.1, data.turn) {
                data.winner = data.turn;
            }
            data.board[_move.0 as usize][_move.1 as usize] = data.turn;
            data.turn = if data.turn == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
            data.last_move_duration = Instant::now().duration_since(data.last_move_time);
            data.last_move_time = Instant::now();
        }
        if data.winner != UNPLAYED_STATE {
            ctx.window().close();
            let window = WindowDesc::new(build_winner())
                .title(LocalizedString::new("Game Over"))
                .resizable(false)
                .window_size(Size::new(600.0, 450.0)
            );
            ctx.new_window(window);
            }
        for p in self.pieces.iter_mut() {
            p.event(ctx, event, data, env);
        }
        //check_rules(data);
        ctx.request_paint();
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        for p in self.pieces.iter_mut() {
            p.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        for p in self.pieces.iter_mut() {
            p.update(ctx, old_data, data, env);
        }
       
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        
        for p in self.pieces.iter_mut() {
            p.layout(layout_ctx, bc, data, env);
        }
        
        let mut size = bc.max();
        
        if size.height != size.width {
            size = Size::new(size.height, size.height);
        }
        if bc.is_width_bounded() && bc.is_height_bounded() {
            size
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let size = ctx.size();
        let board = ctx.size().to_rect();
        ctx.clip(board);
        ctx.fill(board, &Color::rgb8(139, 105, 20));
        let x_delta = ctx.size().width / data.board_size as f64;
        let y_delta = ctx.size().height / data.board_size as f64;
        for i in 0..data.board_size {
            let x = Point::new(x_delta * (i as f64) + (x_delta / 2.0), 0.0);
            let y = Point::new(x_delta * (i as f64) + (x_delta / 2.0), size.height);
            ctx.stroke(
                &Line::new(
                    x,
                    y,
                ),
                &Color::BLACK,
                1.0,
            );
            let x = Point::new(0.0, y_delta * (i as f64) + (y_delta / 2.0));
            let y = Point::new(size.width, y_delta * (i as f64) + (y_delta / 2.0));
            ctx.stroke(
                &Line::new(
                    x,
                    y,
                ),
                &Color::BLACK,
                1.0,
            );
        }
        for p in self.pieces.iter_mut() {
            p.paint(ctx, data, env);
        }

    }
}
