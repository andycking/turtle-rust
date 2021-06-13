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

use std::sync::Arc;

use druid::DelegateCtx;

use crate::common::commands;
use crate::model::app::AppState;
use crate::model::render::RenderTx;
use crate::runtime;

async fn entry_future(input: String, render_tx: Arc<RenderTx>) {
    let res = runtime::entry(input, render_tx);
    if let Err(err) = res {
        eprintln!("{}", err);
    }
}

pub fn go(_ctx: &mut DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.clear();
    let future = entry_future(data.input.to_string(), data.render_tx.clone());
    data.thread_pool.spawn_ok(future);
}

pub fn speed(_ctx: &mut DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::INTERPRETER_SPEED) {
        if data.speed < 16 {
            data.speed *= 2;
        }
    } else {
        if data.speed > 1 {
            data.speed /= 2;
        }
    }
}
