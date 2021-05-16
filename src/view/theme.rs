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

use druid::Color;
use druid::FontDescriptor;
use druid::FontFamily;

pub const MAIN_FILL: Color = Color::WHITE;

pub const CANVAS_WIDTH: f64 = 640.0;
pub const CANVAS_HEIGHT: f64 = 480.0;
pub const CANVAS_BORDER: f64 = 1.0;

pub const INPUT_LINES: f64 = 3.0;
pub const INPUT_FILL: Color = MAIN_FILL;
pub const INPUT_SELECTION_COLOR: Color = Color::rgb8(179, 216, 255);
pub const INPUT_FONT_COLOR: Color = Color::BLACK;
pub const INPUT_FONT_SIZE: f64 = 19.0;
pub const INPUT_FONT: FontDescriptor =
    FontDescriptor::new(FontFamily::MONOSPACE).with_size(INPUT_FONT_SIZE);
