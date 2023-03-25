// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of a timer.


use std::time::Duration;

use druid::widget::prelude::*;
use druid::widget::BackgroundBrush;
use druid::{Color, Point, TimerToken, WidgetPod};

use crate::AppState;

static TIMER_INTERVAL: Duration = Duration::from_millis(10);

pub struct TimerWidget {
    pub timer_id: TimerToken,
    pub simple_box: WidgetPod<u32, SimpleBox>,
    pub pos: Point,
}

impl TimerWidget {
    /// Move the box towards the right, until it reaches the edge,
    /// then reset it to the left but move it to another row.
    fn adjust_box_pos(&mut self, container_size: Size) {
        let box_size = self.simple_box.layout_rect().size();
        self.pos.x += 2.;
        if self.pos.x + box_size.width > container_size.width {
            self.pos.x = 0.;
            self.pos.y += box_size.height;
            if self.pos.y + box_size.height > container_size.height {
                self.pos.y = 0.;
            }
        }
    }
}

impl Widget<AppState> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected => {
                // Start the timer when the application launches
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.adjust_box_pos(ctx.size());
                    ctx.request_layout();
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            }
            _ => (),
        }
        self.simple_box.event(ctx, event, &mut data.time, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.simple_box.lifecycle(ctx, event, &data.time, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, env: &Env) {
        self.simple_box.update(ctx, &data.time, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        self.simple_box.layout(ctx, &bc.loosen(), &data.time, env);
        self.simple_box.set_origin(ctx, self.pos);
        bc.constrain((500.0, 500.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.simple_box.paint(ctx, &data.time, env);
    }
}

pub struct SimpleBox;

impl Widget<u32> for SimpleBox {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut u32, _env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &u32, _env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u32, _data: &u32, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u32,
        _env: &Env,
    ) -> Size {
        bc.constrain((50.0, 50.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &u32, env: &Env) {
        let mut background = if ctx.is_hot() {
            BackgroundBrush::Color(Color::rgb8(200, 55, 55))
        } else {
            BackgroundBrush::Color(Color::rgb8(30, 210, 170))
        };
        background.paint(ctx, data, env);
    }
}