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

use super::lexer_types::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct BinExprNode {
    a: Box<ParserNode>,
    op: LexerOperator,
    b: Box<ParserNode>,
}

impl BinExprNode {
    pub fn new(a: ParserNode, op: LexerOperator, b: ParserNode) -> Self {
        Self {
            a: Box::new(a),
            op,
            b: Box::new(b),
        }
    }

    pub fn a(&self) -> &ParserNode {
        &self.a
    }

    pub fn op(&self) -> LexerOperator {
        self.op
    }

    pub fn b(&self) -> &ParserNode {
        &self.b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallNode {
    name: String,
    args: LexerList,
}

impl CallNode {
    pub fn new(name: &str, args: LexerList) -> Self {
        Self {
            name: String::from(name),
            args,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Backward,
    Forward,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForNode {
    var: String,
    initial: Box<ParserNode>,
    limit: Box<ParserNode>,
    step: Box<ParserNode>,
    list: ParserNodeList,
}

impl ForNode {
    pub fn new(
        var: String,
        initial: ParserNode,
        limit: ParserNode,
        step: ParserNode,
        list: ParserNodeList,
    ) -> Self {
        Self {
            var,
            initial: Box::new(initial),
            limit: Box::new(limit),
            step: Box::new(step),
            list,
        }
    }

    pub fn var(&self) -> &str {
        &self.var
    }

    pub fn initial(&self) -> &ParserNode {
        &self.initial
    }

    pub fn limit(&self) -> &ParserNode {
        &self.limit
    }

    pub fn step(&self) -> &ParserNode {
        &self.step
    }

    pub fn list(&self) -> &ParserNodeList {
        &self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetNode {
    name: String,
    val: Box<ParserNode>,
}

impl LetNode {
    pub fn new(name: String, val: ParserNode) -> Self {
        Self {
            name,
            val: Box::new(val),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn val(&self) -> &ParserNode {
        &self.val
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathOp {
    Atan,
    Cos,
    Log10,
    Ln,
    Round,
    Sin,
    Sqrt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathNode {
    op: MathOp,
    arg: Box<ParserNode>,
}

impl MathNode {
    pub fn new(op: MathOp, arg: ParserNode) -> Self {
        Self {
            op,
            arg: Box::new(arg),
        }
    }

    pub fn op(&self) -> MathOp {
        self.op
    }

    pub fn arg(&self) -> &ParserNode {
        &self.arg
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MoveNode {
    distance: Box<ParserNode>,
    direction: Direction,
}

impl MoveNode {
    pub fn new(distance: ParserNode, direction: Direction) -> Self {
        Self {
            distance: Box::new(distance),
            direction,
        }
    }

    pub fn distance(&self) -> &ParserNode {
        &self.distance
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PenNode {
    Down,
    Erase,
    Paint,
    Reverse,
    Up,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RandomNode {
    max: Box<ParserNode>,
}

impl RandomNode {
    pub fn new(max: ParserNode) -> Self {
        Self { max: Box::new(max) }
    }

    pub fn max(&self) -> &ParserNode {
        &self.max
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepeatNode {
    count: Box<ParserNode>,
    list: ParserNodeList,
}

impl RepeatNode {
    pub fn new(count: ParserNode, list: ParserNodeList) -> Self {
        Self {
            count: Box::new(count),
            list,
        }
    }

    pub fn count(&self) -> &ParserNode {
        &self.count
    }

    pub fn list(&self) -> &ParserNodeList {
        &self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateNode {
    angle: Box<ParserNode>,
    direction: Direction,
}

impl RotateNode {
    pub fn new(angle: ParserNode, direction: Direction) -> Self {
        Self {
            angle: Box::new(angle),
            direction,
        }
    }

    pub fn angle(&self) -> &ParserNode {
        &self.angle
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetHeadingNode {
    angle: Box<ParserNode>,
}

impl SetHeadingNode {
    pub fn new(angle: ParserNode) -> Self {
        Self {
            angle: Box::new(angle),
        }
    }

    pub fn angle(&self) -> &ParserNode {
        &self.angle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPenColorNode {
    color: Box<ParserNode>,
}

impl SetPenColorNode {
    pub fn new(color: ParserNode) -> Self {
        Self {
            color: Box::new(color),
        }
    }

    pub fn color(&self) -> &ParserNode {
        &self.color
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionNode {
    x: Option<Box<ParserNode>>,
    y: Option<Box<ParserNode>>,
}

impl SetPositionNode {
    pub fn new(x: Option<Box<ParserNode>>, y: Option<Box<ParserNode>>) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> Option<&Box<ParserNode>> {
        self.x.as_ref()
    }

    pub fn y(&self) -> Option<&Box<ParserNode>> {
        self.y.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetScreenColorNode {
    color: Box<ParserNode>,
}

impl SetScreenColorNode {
    pub fn new(color: ParserNode) -> Self {
        Self {
            color: Box::new(color),
        }
    }

    pub fn color(&self) -> &ParserNode {
        &self.color
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserNode {
    BinExpr(BinExprNode),
    Call(CallNode),
    Clean,
    ClearScreen,
    Fill,
    For(ForNode),
    Home,
    Let(LetNode),
    List(ParserNodeList),
    Math(MathNode),
    Move(MoveNode),
    Number(f64),
    Pen(PenNode),
    Placeholder,
    Random(RandomNode),
    Repcount,
    Repeat(RepeatNode),
    Rotate(RotateNode),
    SetHeading(SetHeadingNode),
    SetPenColor(SetPenColorNode),
    SetPosition(SetPositionNode),
    SetScreenColor(SetScreenColorNode),
    ShowTurtle(bool),
    Word(String),
}

pub type ParserNodeList = Vec<ParserNode>;

#[derive(Clone, Debug)]
pub struct ParserFuncDef {
    builtin: bool,
    num_args: usize,
    pub list: ParserNodeList,
}

impl ParserFuncDef {
    pub fn new(builtin: bool, num_args: usize, list: ParserNodeList) -> Self {
        Self {
            builtin,
            num_args,
            list,
        }
    }

    pub fn num_args(&self) -> usize {
        self.num_args
    }
}

pub type ParserFuncMap = HashMap<String, ParserFuncDef>;

#[derive(Clone, Debug)]
pub struct ParserOutput {
    pub list: ParserNodeList,
    pub fmap: ParserFuncMap,
}

impl ParserOutput {
    pub fn new(list: ParserNodeList, fmap: ParserFuncMap) -> Self {
        Self { list, fmap }
    }
}
