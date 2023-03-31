use crate::*;

pub struct Board {
    pieces : Vector<BoardPiece>,
    is_old : bool,
}

 impl Board {
    pub fn new(pieces : Vector<BoardPiece>) -> Board {
        Board {
            pieces,
            is_old : false,
        }
    }
 }

/*
    Main Board Interface Widget. 
*/
impl Widget<AppState> for Board {
    
    // Main Event Handler for all game changes.
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env : &Env) {
        // Check if game is over. 
        if game_over(&mut data.board, data.turn, data.captures[(data.turn - 1) as usize]) && !data.winner_opened {
            //ctx.window().close();
            let window = WindowDesc::new(build_winner())
                .title(LocalizedString::new("Game Over"))
                .resizable(false)
                .window_size(Size::new(600.0, 450.0)
            );
            ctx.new_window(window);
            data.winner_opened = true;
            self.is_old = true;
        }
        if self.is_old && !data.winner_opened {
            ctx.window().close();
        }
        // Place Piece on board if player is AI.
        if data.is_ai[(data.turn - 1) as usize] {
            println!("Running algo");
            let _move = alpha_beta_negamax(&mut data.board, data.turn, DEPTH, std::i32::MIN + 2, std::i32::MAX - 2);
            update_board(data, _move.0, _move.1);
        }
        // Give Control to player if User.
        else {
            for p in self.pieces.iter_mut() {
                p.event(ctx, event, data, env);
            }
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

pub fn update_board(data: &mut AppState, x: i32, y: i32) {
    if check_capture(&mut data.board, x, y, data.turn) {
        data.captures[(data.turn - 1) as usize] += 2;
    }
    if is_winner(&mut data.board, x, y, data.turn) {
        data.winner = data.turn;
    }
    data.board[x as usize][y as usize] = data.turn;
    data.turn = if data.turn == PLAYER1_STATE {PLAYER2_STATE} else {PLAYER1_STATE};
    if !data.is_ai[(data.turn - 1) as usize] && data.game_mode == "PvP" {
        println!("Running algo");
        let ai_move = alpha_beta_negamax(&mut data.board, data.turn, DEPTH, std::i32::MIN + 2, std::i32::MAX - 2);
        data.sugested = Some((ai_move.0, ai_move.1));
    } 
    data.last_move_duration = Instant::now().duration_since(data.last_move_time);
    data.last_move_time = Instant::now();
}