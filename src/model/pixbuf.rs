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
    pub fn read(&self, p: Point) -> Color {
        self.read_xy(p.x as usize, p.y as usize)
    }

    fn write_xy_inner(bytes: &mut [u8], x: usize, y: usize, color: &Color) {
        let byte_idx = (y * (DIMS.width as usize) + x) * 4;
        let (red, green, blue, alpha) = color.as_rgba8();
        bytes[byte_idx] = red;
        bytes[byte_idx + 1] = green;
        bytes[byte_idx + 2] = blue;
        bytes[byte_idx + 3] = alpha;
    }

    fn write_xy_inner_clipped(bytes: &mut [u8], x: usize, y: usize, color: &Color) {
        if Self::contains(x, y) {
            Self::write_xy_inner(bytes, x, y, color);
        }
    }

    pub fn write_xy(&mut self, x: usize, y: usize, color: &Color) {
        let bytes = Arc::make_mut(&mut self.bytes);
        Self::write_xy_inner(bytes, x, y, color);
    }

    pub fn write(&mut self, p: Point, color: &Color) {
        self.write_xy(p.x as usize, p.y as usize, color);
    }

    fn screen_xy(x: i32, y: i32) -> (usize, usize) {
        (
            (x + ORIGIN.x as i32) as usize,
            (y + ORIGIN.y as i32) as usize,
        )
    }

    fn contains(x: usize, y: usize) -> bool {
        x >= 0 && x < DIMS.width as usize && y >= 0 && y < DIMS.height as usize
    }

    pub fn line(&mut self, p: &Point, q: &Point, color: &Color) {
        let bytes = Arc::make_mut(&mut self.bytes);

        let x0 = p.x as i32;
        let y0 = -p.y as i32;
        let x1 = q.x as i32;
        let y1 = -q.y as i32;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let adx = (dx.abs() + 1) << 2;
        let ady = (dy.abs() + 1) << 2;

        let sx = if dx > 0 { 1 } else { -1 };
        let sy = if dy > 0 { 1 } else { -1 };

        if adx > ady {
            let mut eps = (ady - adx) >> 1;
            let mut x = x0;
            let mut y = y0;
            loop {
                if sx < 0 && x < x1 {
                    break;
                }
                if sx >= 0 && x > x1 {
                    break;
                }

                let (screen_x, screen_y) = Self::screen_xy(x, y);
                Self::write_xy_inner_clipped(bytes, screen_x, screen_y, color);

                eps += ady;
                if (eps << 1) >= adx {
                    y += sy;
                    eps -= adx;
                }
                x += sx;
            }
        } else {
            let mut eps = (adx - ady) >> 1;
            let mut x = x0;
            let mut y = y0;
            loop {
                if sy < 0 && y < y1 {
                    break;
                }
                if sy >= 0 && y > y1 {
                    break;
                }

                let (screen_x, screen_y) = Self::screen_xy(x, y);
                Self::write_xy_inner_clipped(bytes, screen_x, screen_y, color);

                eps += adx;
                if (eps << 1) >= ady {
                    x += sx;
                    eps -= ady;
                }
                y += sy;
            }
        }
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
