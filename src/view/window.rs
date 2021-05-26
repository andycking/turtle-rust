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

use druid::theme;
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
use druid::Size;
use druid::WidgetExt;
use druid::WindowDesc;

use super::canvas::Canvas;
use super::menu;

use crate::common::constants::*;
use crate::model::app::AppState;

const FONT_SIZE: f64 = 19.0;
const INPUT_WIDTH: f64 = 400.0;

pub fn window() -> WindowDesc<AppState> {
    let ui = build_ui();

    WindowDesc::new(ui)
        .title("Turtle")
        .menu(menu::menu_bar)
        .window_size(window_size())
}

fn build_ui() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_input())
        .with_spacer(1.0)
        .with_child(build_canvas())
        .background(Color::rgb8(208, 208, 208))
        .controller(WindowController {})
}

fn build_canvas() -> impl Widget<AppState> {
    Canvas::new().background(Color::WHITE)
}

fn build_input() -> impl Widget<AppState> {
    let placeholder = "Type your commands here.\n\
        \n\
        When you're ready to make the \n\
        turtle go, press Command-G!";

    Container::new(
        TextBox::multiline()
            .with_placeholder(placeholder)
            .with_text_color(Color::rgb8(88, 110, 117))
            .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_size(FONT_SIZE))
            .with_line_wrapping(false)
            .fix_width(INPUT_WIDTH)
            .expand_height()
            .env_scope(|env, _| {
                env.set(theme::BACKGROUND_LIGHT, Color::rgb8(253, 246, 227));
                env.set(theme::PRIMARY_LIGHT, Color::rgb8(253, 246, 227));
                env.set(theme::BORDER_DARK, Color::rgb8(253, 246, 227));
                env.set(
                    theme::SELECTED_TEXT_BACKGROUND_COLOR,
                    Color::rgb8(238, 232, 213),
                );
                env.set(theme::CURSOR_COLOR, Color::BLACK);
            })
            .lens(AppState::input),
    )
}

fn window_size() -> Size {
    Size::new(DIMS.width + INPUT_WIDTH + 1.0, DIMS.height)
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
