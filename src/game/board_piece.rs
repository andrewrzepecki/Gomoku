use crate::*;


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
    
    // Main Event Handler for User Moves.
    fn event(&mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        _env: &Env
    ) {
        if let Event::MouseDown(event) = event {
            if (self.position - event.pos).hypot() <= self.radius {
                if data.board.is_legal_move(self.x, self.y, data.turn) {
                    update_board(data, self.x, self.y, ctx);
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
        self.state = data.board[(self.x, self.y)];
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

    fn paint(&mut self, 
        ctx: &mut PaintCtx,
        data: &AppState,
        _env: &Env,
    ) {
        let x_delta = ctx.size().width / data.board_size as f64;
        let y_delta = ctx.size().height / data.board_size as f64;
        let x = x_delta * (self.x as f64) + (x_delta / 2.0);
        let y = y_delta * (self.y as f64) + (y_delta / 2.0);
        self.radius = (x_delta + y_delta) / 2.0 / 3.0;
        self.position = Point::new(x, y);
        let color = get_piece_color(&mut data.clone(), self.x, self.y, self.state);
        ctx.fill(Circle::new(self.position, self.radius), &color);
    }
}

// Function to set color of the piece depeding on the AppState.
fn get_piece_color(data : &mut AppState, x: i32, y: i32, state: i32) -> Color {
    let mut color = Color::TRANSPARENT;
    if state != UNPLAYED_STATE {
        color = if state == PLAYER1_STATE {data.colors[data.player1_color as usize]} else {data.colors[data.player2_color as usize]};
    }
    else {
        color = if !data.board.is_legal_move(x, y, data.turn) {Color::rgba8(255, 0, 0, 50)} else {color};
        if data.sugested != None {
            color = if data.sugested.unwrap().0 == x && data.sugested.unwrap().1 == y {Color::rgba8(0, 255, 0, 50)} else {color};
        }
    }
    color  
}
