use crate::*;

pub struct Goban { 
    // Widget child UI elements
    pieces : Vector<BoardPiece>,
    cursor_position: Point,
}

impl Goban {
    pub fn new() -> Self {
        Self {
            pieces : build_pieces(BOARDSIZE),
            cursor_position : Point::new(0.0, 0.0),
        }
    }
}

/*
    Main Board Interface Widget. 
*/

impl Widget<AppState> for Goban {
    
    // Main Event Handler for all game changes.
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env : &Env) {

        if data.turn != Players::Unplayed && data.is_ai[data.turn as usize] {
             let (x, y) = get_best_move(&mut data.board, data.turn);
             data.update_board(x, y);
        }
        else {
            for p in self.pieces.iter_mut() {
                p.event(ctx, event, data, env);
            }
        }
        if let Event::MouseMove(event) = event {
            self.cursor_position = event.pos;
        }
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

    fn update(&mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env
    ) {
        if old_data.board.boards != data.board.boards {  
        }
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
        size = if size.height > size.width {
            Size::new(size.width, size.width)
        } else {
            Size::new(size.height, size.height)
        };
        if bc.is_width_bounded() && bc.is_height_bounded() {
            size
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {

        // Create Board Background
        let size = ctx.size();
        let board = ctx.size().to_rect();
        ctx.clip(board);
        ctx.fill(board, &Color::rgb8(139, 105, 20));
        let x_delta = ctx.size().width / data.board_size as f64;
        let y_delta = ctx.size().height / data.board_size as f64;

        // Create Grid
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

        // Create Cursor Piece.
        let radius = (x_delta + y_delta) / 4.750;
        let color = match data.turn {
            Players::Unplayed => Color::TRANSPARENT,
            Players::PlayerOne => data.colors[data.player_colors[0] as usize],
            Players::PlayerTwo => data.colors[data.player_colors[1] as usize],
        };
        ctx.fill(Circle::new(self.cursor_position, radius), &color);

    }
}