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

use super::pixbuf::PixBuf;
use super::runtime::DrawList;

/// Application state.
#[derive(Clone, Data, Debug, Lens)]
pub struct AppState {
    pub draw_list: Arc<DrawList>,
    pub input: Arc<String>,
    pub output: Arc<String>,
    pub pixels: PixBuf,

    #[data(same_fn = "PartialEq::eq")]
    window_id: druid::WindowId,
}

impl AppState {
    pub fn new(window_id: druid::WindowId) -> Self {
        Self {
            draw_list: DrawList::new().into(),
            input: "".to_string().into(),
            output: "".to_string().into(),
            pixels: Default::default(),
            window_id,
        }
    }
}
