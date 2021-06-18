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

use druid::widget::prelude::*;
use druid::widget::Label;
use druid::widget::LineBreaking;
use druid::Color;
use druid::TextAlignment;
use druid::TimerToken;
use druid::Widget;
use std::time::Duration;

use super::constants::*;
use crate::model::app::AppState;

fn build_console_label() -> Label<AppState> {
    Label::new("")
        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE).with_size(FONT_SIZE))
        .with_text_color(Color::WHITE)
        .with_text_alignment(TextAlignment::Start)
        .with_line_break_mode(LineBreaking::WordWrap)
}

pub struct Console {
    label: Label<AppState>,
    output: String,
    timer_id: TimerToken,
}

impl Console {
    pub fn new() -> Self {
        Self {
            label: build_console_label(),
            output: "".to_string(),
            timer_id: TimerToken::INVALID,
        }
    }

    fn update_output(&mut self, data: &mut AppState) -> bool {
        let output = { data.output.lock().unwrap().clone() };

        if output == self.output {
            return false;
        }

        self.output = output;
        self.label.set_text(self.output.clone());
        true
    }
}

impl Widget<AppState> for Console {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Timer(timer_id) => {
                if self.timer_id == *timer_id {
                    if self.update_output(data) {
                        ctx.request_update();
                    }
                    self.timer_id = ctx.request_timer(Duration::from_millis(100));
                }
            }

            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(Duration::from_millis(100));
            }

            _ => {}
        }

        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.label.layout(ctx, bc, data, env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}
