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
use druid::WidgetExt;
use druid::WindowDesc;

use super::menu;
use super::theme;

use crate::model::app::AppState;

pub fn window() -> WindowDesc<AppState> {
    let ui = build_ui();

    WindowDesc::new(ui)
        .title("Turtle")
        .menu(menu::menu_bar)
        .window_size((window_width(), window_height()))
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(build_input())
        .background(theme::MAIN_FILL)
        .controller(WindowController {})
}

fn build_input() -> impl Widget<AppState> {
    Container::new(
        TextBox::multiline()
            .with_text_color(theme::INPUT_FONT_COLOR)
            .with_font(theme::INPUT_FONT)
            .fix_height(text_height())
            .expand_width()
            .env_scope(|env, _| {
                env.set(druid::theme::BACKGROUND_LIGHT, theme::INPUT_FILL);
                env.set(
                    druid::theme::SELECTED_TEXT_BACKGROUND_COLOR,
                    theme::INPUT_SELECTION_COLOR,
                );
                env.set(druid::theme::CURSOR_COLOR, theme::INPUT_FONT_COLOR);
            })
            .lens(AppState::input),
    )
}

fn window_width() -> f64 {
    theme::CANVAS_WIDTH + (theme::CANVAS_BORDER * 2.0)
}

fn window_height() -> f64 {
    theme::CANVAS_HEIGHT + (theme::CANVAS_BORDER * 2.0) + text_height() + 2.0
}

fn text_height() -> f64 {
    theme::INPUT_LINES * (6.0 + theme::INPUT_FONT_SIZE)
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
