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
use druid::widget::Label;
use druid::widget::MainAxisAlignment;
use druid::widget::TextBox;
use druid::widget::Widget;
use druid::Color;
use druid::FontDescriptor;
use druid::FontFamily;
use druid::Size;
use druid::WidgetExt;
use druid::WindowDesc;

use super::canvas::Canvas;
use super::console::Console;
use super::constants::*;
use super::menu;
use crate::common::constants::*;
use crate::model::app::AppState;
use crate::model::render::RenderRx;

pub fn window(render_rx: RenderRx) -> WindowDesc<AppState> {
    let ui = build_ui(render_rx);

    WindowDesc::new(ui)
        .title("Turtle")
        .menu(menu::menu_bar)
        .window_size(window_size())
}

fn build_ui(render_rx: RenderRx) -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_input())
        .with_child(build_center_pane(render_rx))
        .background(Color::WHITE)
        .controller(WindowController {})
}

fn build_center_pane(render_rx: RenderRx) -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(build_canvas(render_rx))
        .with_spacer(1.0)
        .with_child(build_status_bar())
        .with_spacer(1.0)
        .with_child(build_console())
        .with_default_spacer()
}

fn build_canvas(render_rx: RenderRx) -> impl Widget<AppState> {
    Canvas::new(render_rx).background(Color::BLACK)
}

fn build_input() -> impl Widget<AppState> {
    let placeholder = "Type your instructions in here.\n\
        \n\
        Once you're ready to make the\n\
        turtle carry them out, press\n\
        Command-G.\n\
        \n\
        Look under the Examples menu for\n\
        ideas!";

    Container::new(
        TextBox::multiline()
            .with_placeholder(placeholder)
            .with_text_color(Color::WHITE)
            .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_size(FONT_SIZE))
            .with_line_wrapping(false)
            .fix_width(INPUT_WIDTH)
            .expand_height()
            .env_scope(|env, _| {
                env.set(theme::BACKGROUND_LIGHT, Color::BLACK);
                env.set(theme::PRIMARY_LIGHT, Color::BLACK);
                env.set(theme::BORDER_DARK, Color::BLACK);
                env.set(
                    theme::SELECTED_TEXT_BACKGROUND_COLOR,
                    Color::rgb8(100, 100, 100),
                );
                env.set(theme::CURSOR_COLOR, Color::WHITE);
            })
            .lens(AppState::input),
    )
}

fn build_console() -> impl Widget<AppState> {
    Flex::row()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(Console::new())
        .background(Color::BLACK)
        .fix_width(DIMS.width)
        .fix_height(CONSOLE_HEIGHT)
}

fn build_status_label() -> impl Widget<AppState> {
    Label::new(|data: &AppState, _: &_| {
        format!(
            "commands: {:6}   speed: {:2}",
            data.command_count, data.speed
        )
    })
    .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE).with_size(FONT_SIZE))
    .with_text_color(Color::WHITE)
}

fn build_status_bar() -> impl Widget<AppState> {
    Flex::row()
        .main_axis_alignment(MainAxisAlignment::End)
        .with_child(build_status_label())
        .fix_width(DIMS.width)
        .fix_height(STATUS_BAR_HEIGHT)
        .background(Color::BLACK)
}

fn window_size() -> Size {
    Size::new(
        DIMS.width + INPUT_WIDTH,
        DIMS.height + CONSOLE_HEIGHT + STATUS_BAR_HEIGHT + 2.0,
    )
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
