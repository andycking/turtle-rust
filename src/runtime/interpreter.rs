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
use super::lexer_types::*;
use super::parser_types::*;
use crate::model::render::*;

type ValueList = Vec<Value>;

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Void,
    List(ValueList),
    Number(f64),
}

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

#[derive(Clone, Debug)]
struct Frame {
    pub repcount: usize,
}

impl Frame {
    pub fn new(repcount: usize) -> Self {
        Self { repcount }
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

    pub fn go(&mut self, input: &ParserOutput) -> RuntimeResult {
        let mut frame = Frame::new(0);
        let mut vmap = VarMap::new();
        self.run(&mut frame, &input.fmap, &mut vmap, &input.list)
    }

    fn run(
        &mut self,
        frame: &mut Frame,
        fmap: &ParserFuncMap,
        vmap: &mut VarMap,
        list: &[ParserNode],
    ) -> RuntimeResult {
        for node in list.iter() {
            self.eval_node(frame, fmap, vmap, node)?;
        }

        Ok(())
    }

    fn eval_node(
        &mut self,
        frame: &mut Frame,
        fmap: &ParserFuncMap,
        vmap: &mut VarMap,
        node: &ParserNode,
    ) -> RuntimeResult {
        match node {
            ParserNode::Assign(node) => self.eval_assign(vmap, node),
            ParserNode::Call(node) => self.eval_call(frame, fmap, vmap, node),
            ParserNode::Clean => Ok(self.eval_clean()),
            ParserNode::ClearScreen => self.eval_clear_screen(),
            ParserNode::Home => self.eval_home(),
            ParserNode::Let(node) => self.eval_let(vmap, node),
            ParserNode::Move(node) => self.eval_move(vmap, node),
            ParserNode::Pen(node) => Ok(self.eval_pen(node)),
            ParserNode::Random(node) => self.eval_random(vmap, node),
            ParserNode::Repeat(node) => self.eval_repeat(frame, fmap, vmap, node),
            ParserNode::Rotate(node) => self.eval_rotate(vmap, node),
            ParserNode::SetHeading(node) => self.eval_set_heading(vmap, node),
            ParserNode::SetPenColor(node) => self.eval_set_pen_color(vmap, node),
            ParserNode::SetPosition(node) => self.eval_set_pos(vmap, node),
            ParserNode::SetScreenColor(node) => self.eval_set_screen_color(vmap, node),
        }
    }

    fn eval_assign(&mut self, vmap: &mut VarMap, node: &AssignNode) -> RuntimeResult {
        let value = self.eval_expr(vmap, node.val())?;
        if let Some(var) = vmap.get_mut(node.name()) {
            *var = value;
            Ok(())
        } else {
            let msg = format!("no such variable {}", node.name());
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_call(
        &mut self,
        frame: &mut Frame,
        fmap: &ParserFuncMap,
        vmap: &mut VarMap,
        node: &CallNode,
    ) -> RuntimeResult {
        let name = node.name();
        if let Some(func) = fmap.get(name) {
            let mut child_frame = Frame::new(frame.repcount);
            self.run(&mut child_frame, fmap, vmap, &func.list)
        } else {
            let msg = format!("no such function {}", name);
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_clean(&mut self) {}

    fn eval_clear_screen(&mut self) -> RuntimeResult {
        self.eval_home()?;
        self.eval_clean();
        Ok(())
    }

    fn eval_home(&mut self) -> RuntimeResult {
        self.move_to(Point::ZERO)
    }

    fn eval_let(&mut self, vmap: &mut VarMap, node: &LetNode) -> RuntimeResult {
        let val = self.eval_expr(vmap, node.val())?;
        vmap.insert(node.name().to_string(), val);
        Ok(())
    }

    fn eval_move(&mut self, vmap: &mut VarMap, node: &MoveNode) -> RuntimeResult {
        let distance = self.eval_expr_as_number(vmap, node.distance())?;

        match node.direction() {
            Direction::Forward => self.move_by(distance),
            Direction::Backward => self.move_by(-distance),
            _ => {
                let msg = "movement must be forward or backward".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_pen(&mut self, node: &PenNode) {
        match node {
            PenNode::Down => self.state.pen_down = true,
            PenNode::Up => self.state.pen_down = false,
        }
    }

    fn eval_random(&mut self, vmap: &mut VarMap, expr: &LexerExpr) -> RuntimeResult {
        let max = self.eval_expr_as_number(vmap, expr)?;
        let int = max.round() as u32;
        let num = rand::thread_rng().gen_range(0..=int);
        Ok(())
    }

    fn eval_repeat(
        &mut self,
        _frame: &mut Frame,
        fmap: &ParserFuncMap,
        vmap: &mut VarMap,
        node: &RepeatNode,
    ) -> RuntimeResult {
        let count = self.eval_expr_as_number(vmap, node.count())?;
        let list = node.list();
        let mut child_frame = Frame::new(0);

        for _ in 0..count as usize {
            child_frame.repcount += 1;
            self.run(&mut child_frame, fmap, vmap, list)?;
        }

        Ok(())
    }

    fn eval_rotate(&mut self, vmap: &mut VarMap, node: &RotateNode) -> RuntimeResult {
        let angle = self.eval_expr_as_number(vmap, node.angle())?;

        match node.direction() {
            Direction::Left => {
                self.state.angle -= angle.to_radians();
                Ok(())
            }
            Direction::Right => {
                self.state.angle += angle.to_radians();
                Ok(())
            }
            _ => {
                let msg = "rotation must be right or left".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_set_heading(&mut self, vmap: &mut VarMap, node: &SetHeadingNode) -> RuntimeResult {
        let angle = self.eval_expr_as_number(vmap, node.angle())?;
        self.state.angle = angle.to_radians();
        Ok(())
    }

    fn eval_set_pen_color(&mut self, vmap: &mut VarMap, node: &SetPenColorNode) -> RuntimeResult {
        let val = self.eval_expr(vmap, node.color())?;
        self.state.color = Self::get_color(&self.pal, &val)?;
        Ok(())
    }

    fn eval_set_pos(&mut self, vmap: &mut VarMap, node: &SetPositionNode) -> RuntimeResult {
        let new_x = if let Some(xitem) = node.x() {
            self.eval_expr_as_number(vmap, xitem)?
        } else {
            self.state.pos.x
        };

        let new_y = if let Some(yitem) = node.y() {
            self.eval_expr_as_number(vmap, yitem)?
        } else {
            self.state.pos.y
        };

        self.move_to(Point::new(new_x, new_y))
    }

    fn eval_set_screen_color(
        &mut self,
        vmap: &mut VarMap,
        node: &SetScreenColorNode,
    ) -> RuntimeResult {
        let val = self.eval_expr(vmap, node.color())?;
        self.state.screen_color = Self::get_color(&self.pal, &val)?;
        Ok(())
    }

    fn eval_any_item(&mut self, vmap: &VarMap, item: &LexerAny) -> RuntimeResult<Value> {
        match item {
            LexerAny::LexerBinExpr(expr) => self.eval_bin_expr(vmap, expr),
            LexerAny::LexerExpr(enw) => self.eval_expr(vmap, enw),
            LexerAny::LexerList(list) => self.eval_list(vmap, list),
            LexerAny::LexerNumber(num) => Ok(Value::Number(*num)),
            LexerAny::LexerWord(word) => self.eval_word(vmap, word),
            _ => {
                let msg = format!("cannot evaluate {:?}", item);
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_bin_expr(&mut self, vmap: &VarMap, expr: &LexerBinExpr) -> RuntimeResult<Value> {
        let a = self.eval_expr(vmap, &expr.a())?;
        let op = expr.op();
        let b = self.eval_expr(vmap, &expr.b())?;

        match op {
            LexerOperator::Add => Self::eval_add(&a, &b),
            LexerOperator::Divide => Self::eval_divide(&a, &b),
            LexerOperator::Multiply => Self::eval_multiply(&a, &b),
            LexerOperator::Subtract => Self::eval_subtract(&a, &b),
            _ => {
                let msg = "cannot evaluate assignment in binary expression".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_expr(&mut self, vmap: &VarMap, expr: &LexerExpr) -> RuntimeResult<Value> {
        match expr {
            LexerExpr::LexerBinExpr(bin_expr) => self.eval_bin_expr(vmap, bin_expr),
            LexerExpr::LexerCall(call) => Ok(Value::Number(0.0)),
            LexerExpr::LexerList(list) => self.eval_list(vmap, list),
            LexerExpr::LexerNumber(num) => Ok(Value::Number(*num)),
            LexerExpr::LexerWord(word) => self.eval_word(vmap, word),
        }
    }

    fn eval_expr_as_number(&mut self, vmap: &VarMap, expr: &LexerExpr) -> RuntimeResult<f64> {
        let val = self.eval_expr(vmap, expr)?;
        Self::get_number(&val)
    }

    fn eval_list(&mut self, vmap: &VarMap, list: &[LexerAny]) -> RuntimeResult<Value> {
        let mut out = ValueList::new();
        for item in list.iter() {
            let v = self.eval_any_item(vmap, item)?;
            out.push(v);
        }

        Ok(Value::List(out))
    }

    fn eval_word(&mut self, vmap: &VarMap, word: &str) -> RuntimeResult<Value> {
        if let Some(value) = vmap.get(word) {
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
                let msg = format!("color cannot be void");
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
