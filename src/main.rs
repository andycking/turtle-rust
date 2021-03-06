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

use druid::PlatformError;
use futures::channel::mpsc;

mod common;
mod controller;
mod graphics;
mod model;
mod runtime;
mod view;

use controller::delegate::Delegate;
use model::app::AppState;
use model::render::RenderCommand;
use view::window;

fn main() -> Result<(), PlatformError> {
    let (render_tx, render_rx) = mpsc::unbounded::<RenderCommand>();
    let window = window::window(render_rx);
    let data = AppState::new(render_tx, window.id);

    druid::AppLauncher::with_window(window)
        .delegate(Delegate)
        .launch(data)
}
