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

use crate::common::bits;
use crate::common::constants::*;

#[derive(Clone, Data, Debug)]
pub struct PixBuf {
    width: u32,
    height: u32,
    pub bytes: Arc<Vec<u8>>,
}

impl PixBuf {
    pub fn _width(&self) -> u32 {
        self.width
    }

    pub fn _height(&self) -> u32 {
        self.height
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn clear(&mut self) {
        let mut pixels = Arc::make_mut(&mut self.bytes);
        bits::zero(&mut pixels);
    }

    pub fn read_xy(&self, x: usize, y: usize) -> Color {
        let byte_idx = (y * (self.width as usize) + x) * 4;

        druid::Color::rgba8(
            self.bytes[byte_idx],
            self.bytes[byte_idx + 1],
            self.bytes[byte_idx + 2],
            self.bytes[byte_idx + 3],
        )
    }

    #[inline]
    fn _read(&self, p: Point) -> Color {
        self.read_xy(p.x as usize, p.y as usize)
    }

    fn _write_xy_inner(bytes: &mut [u8], x: usize, y: usize, color: &Color) {
        let byte_idx = (y * (DIMS.width as usize) + x) * 4;
        let (red, green, blue, alpha) = color.as_rgba8();
        bytes[byte_idx] = red;
        bytes[byte_idx + 1] = green;
        bytes[byte_idx + 2] = blue;
        bytes[byte_idx + 3] = alpha;
    }

    pub fn write_xy_inner_clipped(bytes: &mut [u8], x: i32, y: i32, color: &Color) {
        if Self::contains(x, y) {
            Self::_write_xy_inner(bytes, x as usize, y as usize, color);
        }
    }

    pub fn write_xy(&mut self, x: usize, y: usize, color: &Color) {
        let bytes = Arc::make_mut(&mut self.bytes);
        Self::_write_xy_inner(bytes, x, y, color);
    }

    fn _write(&mut self, p: Point, color: &Color) {
        self.write_xy(p.x as usize, p.y as usize, color);
    }

    pub fn screen_xy(x: i32, y: i32) -> (i32, i32) {
        (x + ORIGIN.x as i32, y + ORIGIN.y as i32)
    }

    pub fn contains(x: i32, y: i32) -> bool {
        x >= 0 && x < DIMS.width as i32 && y >= 0 && y < DIMS.height as i32
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
