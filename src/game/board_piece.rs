use crate::*;

#[derive(Clone, Data)]
pub struct BoardPiece {
    x : usize,
    y : usize,
    position : Point,
    radius: f64,
    #[data(eq)]
    state : Players,
}


impl BoardPiece {
    pub fn new(x: usize, y:usize, position : Point, radius: f64, state : Players) -> BoardPiece {
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
             _ctx: &mut EventCtx,
             event: &Event,
             data: &mut AppState,
             _env: &Env
    ) {
        if let Event::MouseDown(event) = event {
            if (self.position - event.pos).hypot() <= self.radius {
                if data.board.move_is_legal(self.x, self.y, data.turn) {
                    data.update_board(self.x, self.y);
                }
            }
        }
        if let Event::MouseMove(event) = event {
            if (self.position - event.pos).hypot() <= self.radius {
                if !data.board.move_is_legal(self.x, self.y, data.turn) {
                    data.change_cursor(false);
                } else {
                    data.change_cursor(true);
                }
                data.candidate_score = get_candidate_score(&mut data.board, self.x, self.x, data.turn);
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
        self.state = data.board.get_state(self.x, self.y);
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
        self.radius = (x_delta + y_delta) / 4.750;
        self.position = Point::new(x, y);
        let color = match self.state {
            Players::Unplayed => Color::TRANSPARENT,
            Players::PlayerOne => data.colors[data.player_colors[0] as usize],
            Players::PlayerTwo => data.colors[data.player_colors[1] as usize],
        };
        ctx.fill(Circle::new(self.position, self.radius), &color);
    }
}

pub fn build_pieces(size : usize) -> Vector<BoardPiece> {
    
    let mut pieces = Vector::new();
    
    for x in 0..size {
        for y in 0..size {
            let point = Point::new(0.0, 0.0);
            let radius = 40.0;
            let piece: BoardPiece = BoardPiece::new(
                x as usize,
                y as usize,
                point,
                radius,
                Players::Unplayed,
            );
            pieces.push_back(piece);
        }
    }
    pieces
}