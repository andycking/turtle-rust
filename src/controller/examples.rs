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

use crate::common::commands;
use crate::model::app::AppState;
use druid::DelegateCtx;
use std::sync::Arc;

pub fn show(_ctx: &mut DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let example = match *cmd.get_unchecked(commands::EXAMPLES) {
        "color-ball" => include_str!("../assets/color-ball.logo"),
        "color-star" => include_str!("../assets/color-star.logo"),
        "fan-flower" => include_str!("../assets/fan-flower.logo"),
        "fill" => include_str!("../assets/fill.logo"),
        "for-loop" => include_str!("../assets/for-loop.logo"),
        "spin-wheel" => include_str!("../assets/spin-wheel.logo"),
        "spiral" => include_str!("../assets/spiral.logo"),
        "squares" => include_str!("../assets/squares.logo"),
        "square-flower" => include_str!("../assets/square-flower.logo"),
        _ => "",
    };

    let input = Arc::make_mut(&mut data.input);
    input.clear();
    input.push_str(example);
}
