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

use druid::Data;
use druid::Lens;
use druid::Point;
use futures::executor::ThreadPool;

use super::pixbuf::PixBuf;
use super::render::RenderTx;

/// Application state.
#[derive(Clone, Data, Debug, Lens)]
pub struct AppState {
    pub input: Arc<String>,
    pub output: Arc<String>,
    pub pixels: PixBuf,
    pub pos: Point,
    pub show_turtle: bool,
    pub speed: u8,
    pub thread_pool: Arc<ThreadPool>,
    pub render_tx: Arc<RenderTx>,

    #[data(same_fn = "PartialEq::eq")]
    window_id: druid::WindowId,
}

impl AppState {
    pub fn new(render_tx: RenderTx, window_id: druid::WindowId) -> Self {
        let thread_pool = ThreadPool::builder()
            .pool_size(1)
            .name_prefix("render-tx")
            .create()
            .expect("Failed to create thread pool");

        Self {
            input: "".to_string().into(),
            output: "".to_string().into(),
            pixels: Default::default(),
            pos: Point::ZERO,
            show_turtle: false,
            speed: 1,
            thread_pool: Arc::new(thread_pool),
            render_tx: Arc::new(render_tx),
            window_id,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.clear();
        self.pos = Point::ZERO;
        self.show_turtle = true;
    }
}
