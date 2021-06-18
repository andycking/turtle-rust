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

use druid::DelegateCtx;

use crate::common::commands;
use crate::model::app::AppState;
use crate::runtime;

pub fn go(_ctx: &mut DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.clear();

    let input = data.input.to_string();
    let output = data.output.clone();
    let render_tx = data.render_tx.clone();

    data.thread_pool.execute(move || {
        let string = match runtime::entry(input, render_tx) {
            Ok(val) => format!("{}", val),
            Err(err) => format!("{}", err),
        };

        let mut guard = output.lock().unwrap();
        guard.clear();
        guard.push_str(&string);
    });
}

pub fn speed(_ctx: &mut DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::INTERPRETER_SPEED) {
        if data.speed < 32 {
            data.speed *= 2;
        }
    } else if data.speed > 1 {
        data.speed /= 2;
    }
}
