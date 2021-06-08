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

use druid::Color;
use druid::DelegateCtx;
use druid::Point;

use crate::model::app::AppState;
use crate::model::runtime::DrawCommand;
use crate::model::runtime::DrawSender;
use crate::runtime;

async fn foo(tx: Arc<DrawSender>) {
    let cmd = DrawCommand::new(1.0, Color::BLACK, 0.0, false, Point::ZERO);
    println!("Weeeeee");
    tx.unbounded_send(cmd);
}

pub fn go(_ctx: &mut DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let future = foo(data.tx.clone());
    data.thread_pool.spawn_ok(future);

    /*match runtime::entry(&data.input) {
        Ok(draw_list) => {
            data.draw_list = Arc::new(draw_list);
        }
        Err(err) => {}
    }*/
}
