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
use druid::Data;
use druid::Point;

use crate::common::constants::*;

#[derive(Clone, Data, Debug)]
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

    #[inline]
    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        y * (self.width as usize) + x
    }

    #[inline]
    fn xy_to_byte_idx(&self, x: usize, y: usize) -> usize {
        self.xy_to_idx(x, y) * 4
    }

    pub fn read_xy(&self, x: usize, y: usize) -> Color {
        let byte_idx = self.xy_to_byte_idx(x, y);

        druid::Color::rgba8(
            self.bytes[byte_idx],
            self.bytes[byte_idx + 1],
            self.bytes[byte_idx + 2],
            self.bytes[byte_idx + 3],
        )
    }

    #[inline]
    pub fn read(&self, p: Point) -> Color {
        self.read_xy(p.x as usize, p.y as usize)
    }

    pub fn write_xy(&mut self, x: usize, y: usize, color: &Color) {
        let byte_idx = self.xy_to_byte_idx(x, y);
        let (red, green, blue, alpha) = color.as_rgba8();

        let pixels = Arc::make_mut(&mut self.bytes);
        pixels[byte_idx] = red;
        pixels[byte_idx + 1] = green;
        pixels[byte_idx + 2] = blue;
        pixels[byte_idx + 3] = alpha;
    }

    pub fn write(&mut self, p: Point, color: &Color) {
        self.write_xy(p.x as usize, p.y as usize, color);
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
