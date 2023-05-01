use crate::*;

pub struct Goban { 
    // Widget child UI elements
    pieces : Vector<BoardPiece>,
    // Main logic for game and rules
}

impl Goban {
    pub fn new(size: usize) -> Self {
        Self {
            pieces : build_pieces(size),
        }
    }

    pub fn dynamic(f: impl Fn(i32) -> i32, arg : i32) -> Self {
        let size: i32 = f(arg);
        Goban::new(size as usize)
    }
}

/*
    Main Board Interface Widget. 
*/

impl Widget<AppState> for Goban {
    
    // Main Event Handler for all game changes.
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env : &Env) {
        
        // Give Control to player if User.
        for p in self.pieces.iter_mut() {
            p.event(ctx, event, data, env);
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



// Update User Interface with a valid move.
/* 
pub fn update_board(data: &mut AppState, x: i32, y: i32, ctx: &mut EventCtx) {
    
    // Stop Timer
    data.last_move_duration = Instant::now().duration_since(data.last_move_time);
    data.last_move_time = Instant::now();
    
    
    // Play Move
    let mut m = BoardMove::new(x, y, data.turn);
    m.set(&mut self.board);
    data.captures[(data.turn - 1) as usize] = data.board.captures[(data.turn - 1) as usize];

    if close_game_conditions(data) {
        close_game_window(data, ctx);
        return;
    }

    // Change turn
    if !data.is_test {
        data.turn = data.board.get_opponent(data.turn); 
    }
    else {
        data.player1_score = get_final_score(&mut data.board, PLAYER1_STATE);
        data.player2_score = get_final_score(&mut data.board, PLAYER2_STATE);
    }

    // Get Sugested Move from AI.
    if !data.is_ai[(data.turn - 1) as usize] && data.game_mode == "PvP" {
        //let best_move = get_best_move(data);
        //let ai_move = alpha_beta_negamax(&mut data.board, data.turn, DEPTH, std::i32::MIN + 2, std::i32::MAX - 2, &mut HashMap::new());
        //let ai_move = mtdf(&mut data.board, data.turn, DEPTH, &mut data.tt, 0);
        data.sugested = Some((-1, -1));
    }
}
    */

/*
fn close_game_conditions(data: &mut AppState) -> bool {
    if !data.is_test && data.board.game_over(data.turn) && !data.winner_opened {
        return true;
    }
    false
}

fn close_game_window(data: &mut AppState, ctx: &mut EventCtx) {
    let window = WindowDesc::new(build_winner())
                .title(LocalizedString::new("Game Over"))
                .resizable(false)
                .window_size(Size::new(600.0, 450.0)
            );
            data.winner = data.board.return_winner();
            data.winner_opened = true;
            ctx.request_paint();
            ctx.new_window(window);
            std::thread::sleep(std::time::Duration::from_secs(3));
            save_tt_table(&mut data.tt);
            ctx.window().close();
}
*/

pub fn update_board(data: &mut AppState, m : &mut BoardMove) {
    data.last_move_duration = Instant::now().duration_since(data.last_move_time);
    data.last_move_time = Instant::now();
    
    // Play Move
    //m.set(&mut data.board);
    //data.turn = data.board.get_opponent(data.turn);
}