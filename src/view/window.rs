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
use druid::widget::Container;
use druid::widget::Controller;
use druid::widget::CrossAxisAlignment;
use druid::widget::Flex;
use druid::widget::TextBox;
use druid::widget::Widget;
use druid::Color;
use druid::FontDescriptor;
use druid::FontFamily;
use druid::WidgetExt;
use druid::WindowDesc;

use super::menu;

use crate::model::app::AppState;

const FONT_SIZE: f64 = 16.0;

pub fn window() -> WindowDesc<AppState> {
    let ui = build_ui();

    WindowDesc::new(ui)
        .title("Turtle")
        .menu(menu::menu_bar)
        .window_size(window_size())
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(build_input())
        .background(Color::WHITE)
        .controller(WindowController {})
}

fn build_input() -> impl Widget<AppState> {
    Container::new(
        TextBox::multiline()
            .with_text_color(Color::BLACK)
            .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_size(FONT_SIZE))
            .fix_height(text_height())
            .expand_width()
            .env_scope(|env, _| {
                env.set(druid::theme::BACKGROUND_LIGHT, Color::WHITE);
                env.set(
                    druid::theme::SELECTED_TEXT_BACKGROUND_COLOR,
                    Color::rgb8(179, 216, 255),
                );
                env.set(druid::theme::CURSOR_COLOR, Color::BLACK);
            })
            .lens(AppState::input),
    )
}

fn window_size() -> (f64, f64) {
    let canvas_width = 640.0 + 2.0;
    let canvas_height = 480.0 + 2.0;
    let input_height = text_height() + 2.0;

    (canvas_width, canvas_height + input_height)
}

fn text_height() -> f64 {
    3.0 * (6.0 + FONT_SIZE)
}

struct WindowController {}

impl<W: Widget<AppState>> Controller<AppState, W> for WindowController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);
    }
}
