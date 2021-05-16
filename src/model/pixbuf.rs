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

use crate::common::constants::*;

#[derive(Clone, druid::Data)]
pub struct PixBuf {
    width: u32,
    height: u32,
    bytes: Arc<Vec<u8>>,
}

impl PixBuf {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for PixBuf {
    fn default() -> Self {
        let dims = DIMS.width as usize * DIMS.height as usize * 4;

        Self {
            width: DIMS.width as u32,
            height: DIMS.height as u32,
            bytes: Arc::new(vec![0; dims]),
        }
    }
}
