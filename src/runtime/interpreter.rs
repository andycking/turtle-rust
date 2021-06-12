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

use std::collections::HashMap;
use std::sync::Arc;

use druid::Color;
use druid::Point;
use rand::Rng;

use super::error::*;
use super::interpreter_types::*;
use super::lexer_types::*;
use super::parser_types::*;
use crate::model::render::*;

type VarMap = HashMap<String, Value>;

type Palette = HashMap<u8, Color>;

#[derive(Clone, Debug)]
struct State {
    angle: f64,
    color: Color,
    pen_down: bool,
    pos: Point,
    screen_color: Color,
}

impl State {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            color: Color::WHITE,
            pen_down: true,
            pos: Point::ZERO,
            screen_color: Color::BLACK,
        }
    }
}

#[derive(Debug)]
struct Frame<'a> {
    pub fmap: &'a ParserFuncMap,
    pub vmap: &'a mut VarMap,
    pub repcount: usize,
}

impl<'a> Frame<'a> {
    pub fn new(fmap: &'a ParserFuncMap, vmap: &'a mut VarMap, repcount: usize) -> Self {
        Self {
            fmap,
            vmap,
            repcount,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Interpreter {
    pal: Palette,
    render_tx: Arc<RenderTx>,
    state: State,
}

impl Interpreter {
    pub fn new(render_tx: Arc<RenderTx>) -> Self {
        let pal = crate::hashmap![
            0 => Color::BLACK,
            1 => Color::BLUE,
            2 => Color::rgb8(0,255,0),        // lime
            3 => Color::AQUA,                 // cyan
            4 => Color::RED,
            5 => Color::FUCHSIA,              // magenta
            6 => Color::YELLOW,
            7 => Color::WHITE,
            8 => Color::rgb8(165, 42, 42),    // brown
            9 => Color::rgb8(210, 180, 140),  // tan
            10 => Color::GREEN,
            11 => Color::rgb8(127, 255, 212), // aqua
            12 => Color::rgb8(250, 128, 114), // salmon
            13 => Color::rgb8(128, 0, 128),   // purple
            14 => Color::rgb8(255, 165, 0),   // orange
            15 => Color::rgb8(128, 128, 128)  // gray
        ];

        Self {
            pal,
            render_tx,
            state: State::new(),
        }
    }

    pub fn go(&mut self, input: &ParserOutput) -> RuntimeResult<Value> {
        let mut vmap = VarMap::new();
        let mut frame = Frame::new(&input.fmap, &mut vmap, 0);
        self.run(&mut frame, &input.list)
    }

    fn run(&mut self, frame: &mut Frame, list: &[ParserNode]) -> RuntimeResult<Value> {
        let mut val = Value::Void;
        for node in list.iter() {
            val = self.eval_node(frame, node)?;
        }
        Ok(val)
    }

    fn eval_node(&mut self, frame: &mut Frame, node: &ParserNode) -> RuntimeResult<Value> {
        match node {
            ParserNode::BinExpr(bin_expr) => self.eval_bin_expr(frame, bin_expr),
            ParserNode::Call(node) => self.eval_call(frame, node),
            ParserNode::Clean => Ok(self.eval_clean()),
            ParserNode::ClearScreen => self.eval_clear_screen(),
            ParserNode::Home => self.eval_home(),
            ParserNode::Let(node) => self.eval_let(frame, node),
            ParserNode::List(node) => self.eval_list(frame, node),
            ParserNode::Move(node) => self.eval_move(frame, node),
            ParserNode::Number(num) => Ok(Value::Number(*num)),
            ParserNode::Pen(node) => Ok(self.eval_pen(node)),
            ParserNode::Random(node) => self.eval_random(frame, node),
            ParserNode::Repcount => Ok(self.eval_repcount(frame)),
            ParserNode::Repeat(node) => self.eval_repeat(frame, node),
            ParserNode::Rotate(node) => self.eval_rotate(frame, node),
            ParserNode::SetHeading(node) => self.eval_set_heading(frame, node),
            ParserNode::SetPenColor(node) => self.eval_set_pen_color(frame, node),
            ParserNode::SetPosition(node) => self.eval_set_pos(frame, node),
            ParserNode::SetScreenColor(node) => self.eval_set_screen_color(frame, node),
            ParserNode::Word(word) => self.eval_word(frame, word),
            _ => Ok(Value::Void),
        }
    }

    fn eval_node_as_number(&mut self, frame: &mut Frame, expr: &ParserNode) -> RuntimeResult<f64> {
        let val = self.eval_node(frame, expr)?;
        Self::get_number(&val)
    }

    fn eval_bin_expr(&mut self, frame: &mut Frame, bin_expr: &BinExprNode) -> RuntimeResult<Value> {
        let a = self.eval_node(frame, &bin_expr.a())?;
        let op = bin_expr.op();
        let b = self.eval_node(frame, &bin_expr.b())?;

        match op {
            LexerOperator::Add => Self::eval_add(&a, &b),
            LexerOperator::Divide => Self::eval_divide(&a, &b),
            LexerOperator::Multiply => Self::eval_multiply(&a, &b),
            LexerOperator::Subtract => Self::eval_subtract(&a, &b),
            _ => {
                let msg = "cannot evaluate operator".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_call(&mut self, frame: &mut Frame, node: &CallNode) -> RuntimeResult<Value> {
        let name = node.name();
        if let Some(func) = frame.fmap.get(name) {
            let mut child_frame = Frame::new(frame.fmap, &mut frame.vmap, frame.repcount);
            self.run(&mut child_frame, &func.list)
        } else {
            let msg = format!("no such function {}", name);
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_clean(&mut self) -> Value {
        Value::Void
    }

    fn eval_clear_screen(&mut self) -> RuntimeResult<Value> {
        self.eval_home()?;
        Ok(self.eval_clean())
    }

    fn eval_home(&mut self) -> RuntimeResult<Value> {
        self.move_to(Point::ZERO)?;
        Ok(Value::Void)
    }

    fn eval_let(&mut self, frame: &mut Frame, node: &LetNode) -> RuntimeResult<Value> {
        let val = self.eval_node(frame, node.val())?;
        frame.vmap.insert(node.name().to_string(), val);
        Ok(Value::Void)
    }

    fn eval_list(&mut self, frame: &mut Frame, list: &[ParserNode]) -> RuntimeResult<Value> {
        let mut out = ValueList::new();
        for item in list.iter() {
            let v = self.eval_node(frame, item)?;
            out.push(v);
        }

        Ok(Value::List(out))
    }

    fn eval_move(&mut self, frame: &mut Frame, node: &MoveNode) -> RuntimeResult<Value> {
        let distance = self.eval_node_as_number(frame, node.distance())?;

        match node.direction() {
            Direction::Forward => {
                self.move_by(distance)?;
                Ok(Value::Void)
            }
            Direction::Backward => {
                self.move_by(-distance)?;
                Ok(Value::Void)
            }
            _ => {
                let msg = "movement must be forward or backward".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_pen(&mut self, node: &PenNode) -> Value {
        match node {
            PenNode::Down => self.state.pen_down = true,
            PenNode::Up => self.state.pen_down = false,
        }
        Value::Void
    }

    fn eval_random(&mut self, frame: &mut Frame, node: &RandomNode) -> RuntimeResult<Value> {
        let max = self.eval_node_as_number(frame, node.max())?;
        let intmax = max.round() as u32;
        let num = rand::thread_rng().gen_range(0..=intmax);
        Ok(Value::Number(num as f64))
    }

    fn eval_repcount(&mut self, frame: &mut Frame) -> Value {
        Value::Number(frame.repcount as f64)
    }

    fn eval_repeat(&mut self, frame: &mut Frame, node: &RepeatNode) -> RuntimeResult<Value> {
        let count = self.eval_node_as_number(frame, node.count())?;
        let list = node.list();
        let mut child_frame = Frame::new(frame.fmap, &mut frame.vmap, 0);

        for _ in 0..count as usize {
            child_frame.repcount += 1;
            self.run(&mut child_frame, list)?;
        }

        Ok(Value::Void)
    }

    fn eval_rotate(&mut self, frame: &mut Frame, node: &RotateNode) -> RuntimeResult<Value> {
        let angle = self.eval_node_as_number(frame, node.angle())?;

        match node.direction() {
            Direction::Left => {
                self.state.angle -= angle.to_radians();
                Ok(Value::Void)
            }
            Direction::Right => {
                self.state.angle += angle.to_radians();
                Ok(Value::Void)
            }
            _ => {
                let msg = "rotation must be right or left".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_set_heading(
        &mut self,
        frame: &mut Frame,
        node: &SetHeadingNode,
    ) -> RuntimeResult<Value> {
        let angle = self.eval_node_as_number(frame, node.angle())?;
        self.state.angle = angle.to_radians();
        Ok(Value::Void)
    }

    fn eval_set_pen_color(
        &mut self,
        frame: &mut Frame,
        node: &SetPenColorNode,
    ) -> RuntimeResult<Value> {
        let val = self.eval_node(frame, node.color())?;
        self.state.color = Self::get_color(&self.pal, &val)?;
        Ok(Value::Void)
    }

    fn eval_set_pos(&mut self, frame: &mut Frame, node: &SetPositionNode) -> RuntimeResult<Value> {
        let new_x = if let Some(xitem) = node.x() {
            self.eval_node_as_number(frame, xitem)?
        } else {
            self.state.pos.x
        };

        let new_y = if let Some(yitem) = node.y() {
            self.eval_node_as_number(frame, yitem)?
        } else {
            self.state.pos.y
        };

        self.move_to(Point::new(new_x, new_y))?;

        Ok(Value::Void)
    }

    fn eval_set_screen_color(
        &mut self,
        frame: &mut Frame,
        node: &SetScreenColorNode,
    ) -> RuntimeResult<Value> {
        let val = self.eval_node(frame, node.color())?;
        self.state.screen_color = Self::get_color(&self.pal, &val)?;
        Ok(Value::Void)
    }

    fn eval_word(&mut self, frame: &mut Frame, word: &str) -> RuntimeResult<Value> {
        if let Some(value) = frame.vmap.get(word) {
            Ok(value.clone())
        } else {
            let msg = format!("no such variable {}", word);
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn err_eval_bin_expr(a: &Value, b: &Value) -> RuntimeResult<Value> {
        let msg = format!("cannot evaluate {:?} {:?}", a, b);
        Err(RuntimeError::Interpreter(msg))
    }

    fn eval_add(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num + b_num)),
                _ => Self::err_eval_bin_expr(a, b),
            },
            Value::List(a_list) => match b {
                Value::List(b_list) => {
                    let mut merged = ValueList::new();
                    merged.extend_from_slice(&a_list);
                    merged.extend_from_slice(&b_list);
                    Ok(Value::List(merged))
                }
                Value::Number(b_num) => {
                    let mut merged = ValueList::new();
                    merged.extend_from_slice(&a_list);
                    merged.push(Value::Number(*b_num));
                    Ok(Value::List(merged))
                }
                _ => Self::err_eval_bin_expr(a, b),
            },
            _ => Self::err_eval_bin_expr(a, b),
        }
    }

    fn eval_divide(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(other_num) => Ok(Value::Number(a_num / other_num)),
                _ => Self::err_eval_bin_expr(a, b),
            },
            _ => Self::err_eval_bin_expr(a, b),
        }
    }

    fn eval_multiply(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num * b_num)),
                _ => Self::err_eval_bin_expr(a, b),
            },
            _ => Self::err_eval_bin_expr(a, b),
        }
    }

    fn eval_subtract(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num - b_num)),
                _ => Self::err_eval_bin_expr(a, b),
            },
            _ => Self::err_eval_bin_expr(a, b),
        }
    }

    fn get_color_component(val: &Value) -> RuntimeResult<u8> {
        let comp = Self::get_number(val)?;
        if (0.0..=255.0).contains(&comp) {
            Ok(comp as u8)
        } else {
            let msg = format!("color component out of bounds {}", comp);
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn get_color(pal: &Palette, val: &Value) -> RuntimeResult<Color> {
        match val {
            Value::List(list) => {
                Self::vlist_expect(&list, 3)?;
                let red = Self::get_color_component(&list[0])?;
                let green = Self::get_color_component(&list[1])?;
                let blue = Self::get_color_component(&list[2])?;

                Ok(Color::rgb8(red as u8, green as u8, blue as u8))
            }

            Value::Number(num) => {
                let idx = *num as u8;
                if let Some(color) = pal.get(&idx) {
                    Ok(color.clone())
                } else {
                    let msg = format!("invalid palette index {}", idx);
                    Err(RuntimeError::Interpreter(msg))
                }
            }

            _ => {
                let msg = "color cannot be void".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn get_number(val: &Value) -> RuntimeResult<f64> {
        if let Value::Number(num) = val {
            Ok(*num)
        } else {
            let msg = "expected a number".to_string();
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn angle(p: &Point, other: &Point) -> f64 {
        other.y.atan2(other.x) - p.y.atan2(p.x)
    }

    fn move_by(&mut self, distance: f64) -> RuntimeResult {
        let angle = (90.0_f64).to_radians() - self.state.angle;
        let p = Point::new(
            (self.state.pos.x + distance * angle.cos()).round(),
            (self.state.pos.y + distance * angle.sin()).round(),
        );
        self.move_to_inner(angle, p)?;
        self.state.pos = p;
        Ok(())
    }

    fn move_to(&mut self, p: Point) -> RuntimeResult {
        let angle = Self::angle(&p, &self.state.pos);
        self.move_to_inner(angle, p)?;
        self.state.pos = p;
        Ok(())
    }

    fn move_to_inner(&mut self, angle: f64, p: Point) -> RuntimeResult {
        let move_to = MoveTo::new(angle, self.state.color.clone(), 0.0, self.state.pen_down, p);

        let cmd = RenderCommand::MoveTo(move_to);
        self.render_tx.unbounded_send(cmd)?;

        Ok(())
    }

    fn vlist_expect(list: &[Value], n: usize) -> RuntimeResult {
        if list.len() < n {
            let msg = format!("{} items expected", n);
            Err(RuntimeError::Interpreter(msg))
        } else {
            Ok(())
        }
    }
}
