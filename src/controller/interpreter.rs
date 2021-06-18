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
use crate::common::constants::MAX_SPEED;
use crate::common::constants::MIN_SPEED;
use crate::model::app::AppState;
use crate::runtime;
use druid::DelegateCtx;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

fn set_output(output: &Arc<Mutex<String>>, string: &str) {
    let mut output_guard = output.lock().unwrap();
    output_guard.clear();
    output_guard.push_str(&string);
}

fn set_running(running: &Arc<AtomicBool>) -> bool {
    match running.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
        Ok(false) => true, // Original value replaced.
        _ => false,        // Anything else means it's still running.
    }
}

fn clear_running(running: &Arc<AtomicBool>) {
    running
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire)
        .unwrap();
}

fn go_inner(data: &mut AppState) {
    data.clear();

    let input = data.input.to_string();
    let output = data.output.clone();
    let render_tx = data.render_tx.clone();
    let running = data.running.clone();
    let speed = data.speed.clone();

    data.thread_pool.execute(move || {
        let string = match runtime::entry(input, render_tx, speed) {
            Ok(val) => format!("{}", val),
            Err(err) => format!("{}", err),
        };

        set_output(&output, &string);
        clear_running(&running);
    });
}

pub fn go(_ctx: &mut DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    if set_running(&data.running) {
        go_inner(data);
    }
}

pub fn speed(_ctx: &mut DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let faster = *cmd.get_unchecked(commands::INTERPRETER_SPEED);

    data.speed
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
            if faster {
                Some(std::cmp::min(x * 2, MAX_SPEED))
            } else {
                Some(std::cmp::max(x / 2, MIN_SPEED))
            }
        });
}
