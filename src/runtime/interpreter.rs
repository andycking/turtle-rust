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

use druid::Color;
use druid::Point;

use super::error::*;
use super::lexer_types::*;
use super::parser_types::*;
use super::types::*;

type ValueList = Vec<Value>;

#[derive(Clone, Debug, PartialEq)]
enum Value {
    List(ValueList),
    Number(f64),
}

type VarMap = HashMap<String, Value>;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

type Palette = HashMap<u8, Color>;

#[derive(Clone, Debug)]
pub struct State {
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
pub struct Interpreter {
    draw_list: DrawList,
    pal: Palette,
    state: State,
}

impl Interpreter {
    pub fn new() -> Self {
        let pal = hashmap![
            0 => Color::BLACK,
            1 => Color::BLUE,
            2 => Color::GREEN,
            3 => Color::AQUA,
            4 => Color::RED,
            5 => Color::MAROON,
            6 => Color::YELLOW,
            7 => Color::WHITE
        ];

        Self {
            draw_list: DrawList::new(),
            pal,
            state: State::new(),
        }
    }

    pub fn go(&mut self, input: &ParserOutput) -> RuntimeResult<DrawList> {
        let mut vmap = VarMap::new();
        self.run(&input.fmap, &mut vmap, &input.list)?;
        Ok(self.draw_list.to_owned())
    }

    fn run(&mut self, fmap: &FuncMap, vmap: &mut VarMap, list: &NodeList) -> RuntimeResult {
        for node in list.iter() {
            match node {
                Node::Assign(node) => self.eval_assign(vmap, node)?,
                Node::Call(node) => self.eval_call(fmap, vmap, node)?,
                Node::Clean => self.eval_clean(),
                Node::ClearScreen => self.eval_clear_screen(),
                Node::Home => self.eval_home(),
                Node::Let(node) => self.eval_let(vmap, node)?,
                Node::Move(node) => self.eval_move(vmap, node)?,
                Node::Pen(node) => self.eval_pen(node),
                Node::Repeat(node) => self.eval_repeat(fmap, vmap, node)?,
                Node::Rotate(node) => self.eval_rotate(vmap, node)?,
                Node::SetHeading(node) => self.eval_set_heading(vmap, node)?,
                Node::SetPenColor(node) => self.eval_set_pen_color(vmap, node)?,
                Node::SetPosition(node) => self.eval_set_pos(vmap, node)?,
                Node::SetScreenColor(node) => self.eval_set_screen_color(vmap, node)?,
            }
        }

        Ok(())
    }

    fn eval_assign(&mut self, vmap: &mut VarMap, node: &AssignNode) -> RuntimeResult {
        let value = self.eval_expr_num_word(vmap, node.val())?;
        if let Some(var) = vmap.get_mut(node.name()) {
            *var = value;
            Ok(())
        } else {
            let msg = format!("No such variable {}", node.name());
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_call(&mut self, fmap: &FuncMap, vmap: &mut VarMap, node: &CallNode) -> RuntimeResult {
        let name = node.name();
        if let Some(func) = fmap.get(name.name()) {
            self.run(fmap, vmap, func)
        } else {
            let msg = format!("No such function {}", name.name());
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_clean(&mut self) {}

    fn eval_clear_screen(&mut self) {
        self.eval_home();
        self.eval_clean();
    }

    fn eval_home(&mut self) {
        self.move_to(Point::ZERO);
    }

    fn eval_let(&mut self, vmap: &mut VarMap, node: &LetNode) -> RuntimeResult {
        let val = self.eval_expr_num_word(vmap, node.val())?;
        vmap.insert(node.name().to_string(), val);
        Ok(())
    }

    fn eval_move(&mut self, vmap: &mut VarMap, node: &MoveNode) -> RuntimeResult {
        let distance = self.eval_expr_num_word_as_number(vmap, node.distance())?;

        match node.direction() {
            Direction::Forward => {
                self.move_by(distance);
                Ok(())
            }
            Direction::Backward => {
                self.move_by(-distance);
                Ok(())
            }
            _ => {
                let msg = "Movement must be forward or backward".to_string();
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

    fn eval_repeat(
        &mut self,
        fmap: &FuncMap,
        vmap: &mut VarMap,
        node: &RepeatNode,
    ) -> RuntimeResult {
        let count = self.eval_expr_num_word_as_number(vmap, node.count())?;
        let list = node.list();

        for _ in 0..count as usize {
            self.run(fmap, vmap, list)?;
        }

        Ok(())
    }

    fn eval_rotate(&mut self, vmap: &mut VarMap, node: &RotateNode) -> RuntimeResult {
        let angle = self.eval_expr_num_word_as_number(vmap, node.angle())?;

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
                let msg = "Rotation must be right or left".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_set_heading(&mut self, vmap: &mut VarMap, node: &SetHeadingNode) -> RuntimeResult {
        let angle = self.eval_expr_num_word_as_number(vmap, node.angle())?;
        self.state.angle = angle.to_radians();
        Ok(())
    }

    fn eval_set_pen_color(&mut self, vmap: &mut VarMap, node: &SetPenColorNode) -> RuntimeResult {
        let val = self.eval_list_num_word(vmap, node.color())?;
        self.state.color = Self::get_color(&self.pal, &val)?;
        Ok(())
    }

    fn eval_set_pos(&mut self, vmap: &mut VarMap, node: &SetPositionNode) -> RuntimeResult {
        let new_x = if let Some(xitem) = node.x() {
            self.eval_expr_num_word_as_number(vmap, xitem)?
        } else {
            self.state.pos.x
        };

        let new_y = if let Some(yitem) = node.y() {
            self.eval_expr_num_word_as_number(vmap, yitem)?
        } else {
            self.state.pos.y
        };

        self.move_to(Point::new(new_x, new_y));

        Ok(())
    }

    fn eval_set_screen_color(
        &mut self,
        vmap: &mut VarMap,
        node: &SetScreenColorNode,
    ) -> RuntimeResult {
        let val = self.eval_list_num_word(vmap, node.color())?;
        self.state.screen_color = Self::get_color(&self.pal, &val)?;
        Ok(())
    }

    fn eval_any_item(&mut self, vmap: &VarMap, item: &AnyItem) -> RuntimeResult<Value> {
        match item {
            AnyItem::Expression(expr) => self.eval_expr(vmap, expr),
            AnyItem::ExprNumWord(enw) => self.eval_expr_num_word(vmap, enw),
            AnyItem::List(list) => self.eval_list(vmap, list),
            AnyItem::ListNumWord(lnw) => self.eval_list_num_word(vmap, lnw),
            AnyItem::Number(num) => Ok(Value::Number(num.val())),
            AnyItem::Word(word) => self.eval_word(vmap, word),
            _ => {
                let msg = "Can't evaluate item".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_expr(&mut self, vmap: &VarMap, expr: &Expression) -> RuntimeResult<Value> {
        let a = self.eval_expr_num_word(vmap, &expr.a())?;
        let op = expr.op();
        let b = self.eval_expr_num_word(vmap, &expr.b())?;

        match op {
            Operator::Add => Self::eval_add(&a, &b),
            Operator::Divide => Self::eval_divide(&a, &b),
            Operator::Multiply => Self::eval_multiply(&a, &b),
            Operator::Subtract => Self::eval_subtract(&a, &b),
            _ => {
                let msg = "Can't evaluate assignment as part of expression".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_expr_num_word(
        &mut self,
        vmap: &VarMap,
        expr_num_word: &ExprNumWord,
    ) -> RuntimeResult<Value> {
        match expr_num_word {
            ExprNumWord::Expression(expr) => self.eval_expr(vmap, expr),
            ExprNumWord::Number(num) => Ok(Value::Number(num.val())),
            ExprNumWord::Word(word) => self.eval_word(vmap, word),
        }
    }

    fn eval_expr_num_word_as_number(
        &mut self,
        vmap: &VarMap,
        expr_num_word: &ExprNumWord,
    ) -> RuntimeResult<f64> {
        let val = self.eval_expr_num_word(vmap, expr_num_word)?;
        Self::get_number(&val)
    }

    fn eval_list(&mut self, vmap: &VarMap, list: &List) -> RuntimeResult<Value> {
        let mut out = ValueList::new();
        for item in list.iter() {
            let v = self.eval_any_item(vmap, item)?;
            out.push(v);
        }

        Ok(Value::List(out))
    }

    fn eval_list_num_word(
        &mut self,
        vmap: &VarMap,
        list_num_word: &ListNumWord,
    ) -> RuntimeResult<Value> {
        match list_num_word {
            ListNumWord::List(list) => self.eval_list(vmap, list),
            ListNumWord::Number(num) => Ok(Value::Number(num.val())),
            ListNumWord::Word(word) => self.eval_word(vmap, word),
        }
    }

    fn eval_word(&mut self, vmap: &VarMap, word: &Word) -> RuntimeResult<Value> {
        if let Some(value) = vmap.get(word.name()) {
            Ok(value.clone())
        } else {
            let msg = format!("No such variable {}", word.name());
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn eval_add(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num + b_num)),
                _ => {
                    let msg = "Can't add a number and a list".to_string();
                    Err(RuntimeError::Interpreter(msg))
                }
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
            },
        }
    }

    fn eval_divide(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(other_num) => Ok(Value::Number(a_num / other_num)),
                _ => {
                    let msg = "Can't divide a number and a list".to_string();
                    Err(RuntimeError::Interpreter(msg))
                }
            },
            Value::List(_) => {
                let msg = "Can't divide two lists".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_multiply(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num * b_num)),
                _ => {
                    let msg = "Can't multiply a number and a list".to_string();
                    Err(RuntimeError::Interpreter(msg))
                }
            },
            Value::List(_) => {
                let msg = "Can't multiply two lists".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn eval_subtract(a: &Value, b: &Value) -> RuntimeResult<Value> {
        match a {
            Value::Number(a_num) => match b {
                Value::Number(b_num) => Ok(Value::Number(a_num - b_num)),
                _ => {
                    let msg = "Can't subtract a list from a number".to_string();
                    Err(RuntimeError::Interpreter(msg))
                }
            },
            Value::List(_) => {
                let msg = "Can't subtract two lists".to_string();
                Err(RuntimeError::Interpreter(msg))
            }
        }
    }

    fn get_color_component(val: &Value) -> RuntimeResult<u8> {
        let comp = Self::get_number(val)?;
        if (0.0..=255.0).contains(&comp) {
            Ok(comp as u8)
        } else {
            let msg = format!("Color component out of bounds {}", comp);
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
                    let msg = format!("Invalid palette index {}", idx);
                    Err(RuntimeError::Interpreter(msg))
                }
            }
        }
    }

    fn get_number(val: &Value) -> RuntimeResult<f64> {
        if let Value::Number(num) = val {
            Ok(*num)
        } else {
            let msg = "Expected a number".to_string();
            Err(RuntimeError::Interpreter(msg))
        }
    }

    fn move_to(&mut self, p: Point) {
        let angle = angle(&p, &self.state.pos);
        self.push_cmd(angle, p);
        self.state.pos = p;
    }

    fn move_by(&mut self, distance: f64) {
        let angle = (90.0_f64).to_radians() - self.state.angle;
        let p = Point::new(
            (self.state.pos.x + distance * angle.cos()).round(),
            (self.state.pos.y + distance * angle.sin()).round(),
        );
        self.push_cmd(angle, p);
        self.state.pos = p;
    }

    fn push_cmd(&mut self, angle: f64, pos: Point) {
        self.draw_list.push(DrawCommand::new(
            angle,
            self.state.color.clone(),
            0.0,
            self.state.pen_down,
            pos,
        ));
    }

    fn vlist_expect(list: &ValueList, n: usize) -> RuntimeResult {
        if list.len() < n {
            let msg = format!("Expected a list of at least {} items", n);
            Err(RuntimeError::Interpreter(msg))
        } else {
            Ok(())
        }
    }
}

fn angle(p: &Point, other: &Point) -> f64 {
    other.y.atan2(other.x) - p.y.atan2(p.x)
}
