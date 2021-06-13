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
use druid::Data;
use druid::Point;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::UnboundedSender;

pub const PEN_FLAGS_MASK_VIS: u32 = 0xff;
pub const PEN_FLAGS_DOWN: u32 = 1 << 0;
pub const PEN_FLAGS_UP: u32 = 1 << 1;
pub const PEN_FLAGS_PAINT: u32 = 1 << 8;
pub const PEN_FLAGS_ERASE: u32 = 1 << 9;
pub const PEN_FLAGS_REVERSE: u32 = 1 << 10;
pub const PEN_FLAGS_DEFAULT: u32 = PEN_FLAGS_DOWN | PEN_FLAGS_PAINT;

pub fn is_pen_down(flags: u32) -> bool {
    flags & PEN_FLAGS_DOWN == PEN_FLAGS_DOWN
}

pub fn pen_down(flags: u32) -> u32 {
    (flags & !PEN_FLAGS_MASK_VIS) | PEN_FLAGS_DOWN
}

pub fn pen_up(flags: u32) -> u32 {
    (flags & !PEN_FLAGS_MASK_VIS) | PEN_FLAGS_UP
}

pub fn is_pen_erase(flags: u32) -> bool {
    flags & PEN_FLAGS_ERASE == PEN_FLAGS_ERASE
}

#[derive(Clone, Data, Debug, PartialEq)]
pub struct MoveTo {
    angle: f64,
    pub color: Color,
    distance: f64,
    pub pen_flags: u32,
    pub pos: Point,
}

impl MoveTo {
    pub fn new(angle: f64, color: Color, distance: f64, pen_flags: u32, pos: Point) -> Self {
        Self {
            angle,
            color,
            distance,
            pen_flags,
            pos,
        }
    }
}

#[derive(Clone, Data, Debug, PartialEq)]
pub enum RenderCommand {
    MoveTo(MoveTo),
    ShowTurtle(bool),
}

pub type RenderRx = UnboundedReceiver<RenderCommand>;
pub type RenderTx = UnboundedSender<RenderCommand>;

#[cfg(test)]
mod tests {
    use futures::channel::mpsc;

    use super::*;
    use crate::model::render::RenderCommand;

    #[test]
    fn it_sets_pen_down() {
        let input = PEN_FLAGS_UP | PEN_FLAGS_ERASE;
        let got = pen_down(input);
        assert_eq!(got, PEN_FLAGS_DOWN | PEN_FLAGS_ERASE);
    }

    #[test]
    fn it_sets_pen_up() {
        let input = PEN_FLAGS_DOWN | PEN_FLAGS_REVERSE;
        let got = pen_up(input);
        assert_eq!(got, PEN_FLAGS_UP | PEN_FLAGS_REVERSE);
    }
}
