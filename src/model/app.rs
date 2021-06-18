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

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::Mutex;

use druid::Data;
use druid::Lens;
use druid::Point;
use threadpool::ThreadPool;

use super::pixbuf::PixBuf;
use super::render::RenderTx;

/// Application state.
#[derive(Clone, Data, Debug, Lens)]
pub struct AppState {
    pub command_count: u32,
    pub input: Arc<String>,
    pub output: Arc<Mutex<String>>,
    pub pixels: PixBuf,
    pub pos: Point,
    pub running: Arc<AtomicBool>,
    pub show_turtle: bool,
    pub speed: Arc<AtomicU32>,
    pub thread_pool: Arc<ThreadPool>,
    pub render_tx: Arc<RenderTx>,

    #[data(same_fn = "PartialEq::eq")]
    window_id: druid::WindowId,
}

impl AppState {
    pub fn new(render_tx: RenderTx, window_id: druid::WindowId) -> Self {
        Self {
            command_count: 0,
            input: "".to_string().into(),
            output: Arc::new(Mutex::new("".to_string())),
            pixels: Default::default(),
            pos: Point::ZERO,
            running: Arc::new(AtomicBool::new(false)),
            show_turtle: false,
            speed: Arc::new(AtomicU32::new(4)),
            thread_pool: Arc::new(ThreadPool::new(1)),
            render_tx: Arc::new(render_tx),
            window_id,
        }
    }

    pub fn clear(&mut self) {
        self.command_count = 0;
        self.pixels.clear();
        self.pos = Point::ZERO;
        self.show_turtle = true;
    }
}
