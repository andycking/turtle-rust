// Copyright 2021 Andy King
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

use std::time::Duration;

use druid::piet::ImageFormat;
use druid::piet::InterpolationMode;
use druid::widget::prelude::*;
use druid::Rect;
use druid::TimerToken;
use druid::Widget;

use crate::common::constants::*;
use crate::model::app::AppState;
use crate::model::render::*;

pub struct Canvas {
    render_rx: RenderRx,
    timer_id: TimerToken,
}

impl Canvas {
    pub fn new(render_rx: RenderRx) -> Self {
        Self {
            render_rx,
            timer_id: TimerToken::INVALID,
        }
    }

    pub fn render(&mut self, data: &mut AppState) {
        if let Ok(Some(cmd)) = self.render_rx.try_next() {
            match cmd {
                RenderCommand::MoveTo(move_to) => {
                    let p = data.pos;
                    let q = move_to.pos;
                    data.pixels.line(&p, &q, &move_to.color);
                    data.pos = q;
                }
            }
        }
    }
}

impl Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::Timer(timer_id) => {
                if self.timer_id == *timer_id {
                    self.render(data);
                    self.timer_id = ctx.request_timer(Duration::from_millis(20));
                }
            }

            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(Duration::from_millis(20));
            }

            _ => {}
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

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.constrain(DIMS)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let image = ctx
            .make_image(
                DIMS.width as usize,
                DIMS.height as usize,
                &data.pixels.bytes(),
                ImageFormat::RgbaSeparate,
            )
            .unwrap();
        let rect = Rect::from_origin_size((0.0, 0.0), DIMS);
        ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
    }
}
