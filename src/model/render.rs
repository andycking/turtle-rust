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

#[derive(Clone, Data, Debug, PartialEq)]
pub struct MoveTo {
    angle: f64,
    pub color: Color,
    distance: f64,
    pub pen_down: bool,
    pub pos: Point,
}

impl MoveTo {
    pub fn new(angle: f64, color: Color, distance: f64, pen_down: bool, pos: Point) -> Self {
        Self {
            angle,
            color,
            distance,
            pen_down,
            pos,
        }
    }
}

#[derive(Clone, Data, Debug, PartialEq)]
pub enum RenderCommand {
    MoveTo(MoveTo),
}

pub type RenderRx = UnboundedReceiver<RenderCommand>;
pub type RenderTx = UnboundedSender<RenderCommand>;
