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

use std::collections::VecDeque;
use std::sync::Arc;

use druid::Color;
use druid::Point;

use crate::model::pixbuf::PixBuf;

pub fn line(pixels: &mut PixBuf, p: &Point, q: &Point, color: &Color) {
    let bytes = Arc::make_mut(&mut pixels.bytes);

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

            let (screen_x, screen_y) = PixBuf::screen_xy(x, y);
            PixBuf::write_xy_inner_clipped(bytes, screen_x, screen_y, color);

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

            let (screen_x, screen_y) = PixBuf::screen_xy(x, y);
            PixBuf::write_xy_inner_clipped(bytes, screen_x, screen_y, color);

            eps += adx;
            if (eps << 1) >= ady {
                x += sx;
                eps -= ady;
            }
            y += sy;
        }
    }
}

pub fn flood_fill(pixels: &mut PixBuf, pos: &druid::Point, color: &Color) {
    let (x, y) = PixBuf::screen_xy(pos.x as i32, -pos.y as i32);
    if !PixBuf::contains(x, y) {
        return;
    }

    let start_color = pixels.read_xy(x as usize, y as usize);
    if start_color == *color {
        return;
    }

    let mut q: VecDeque<Point> = VecDeque::new();
    q.push_back(Point::new(x as f64, y as f64));
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let x = node.x as usize;
        let y = node.y as usize;

        if start_color == pixels.read_xy(x, y) {
            pixels.write_xy(x, y, color);

            let left = node - (1.0, 0.0);
            if PixBuf::contains(left.x as i32, left.y as i32) {
                q.push_back(left);
            }

            let right = node + (1.0, 0.0);
            if PixBuf::contains(right.x as i32, right.y as i32) {
                q.push_back(right);
            }

            let up = node - (0.0, 1.0);
            if PixBuf::contains(up.x as i32, up.y as i32) {
                q.push_back(up);
            }

            let down = node + (0.0, 1.0);
            if PixBuf::contains(down.x as i32, down.y as i32) {
                q.push_back(down);
            }
        }
    }
}
