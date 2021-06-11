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

use super::lexer_types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AssignNode {
    name: String,
    val: LexerExpr,
}

impl AssignNode {
    pub fn new(name: String, val: LexerExpr) -> Self {
        Self { name, val }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn val(&self) -> &LexerExpr {
        &self.val
    }
}

pub type CallNode = LexerCall;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Backward,
    Forward,
    Right,
}

pub type LetNode = AssignNode;

#[derive(Clone, Debug, PartialEq)]
pub struct MoveNode {
    distance: LexerExpr,
    direction: Direction,
}

impl MoveNode {
    pub fn new(distance: LexerExpr, direction: Direction) -> Self {
        Self {
            distance,
            direction,
        }
    }

    pub fn distance(&self) -> &LexerExpr {
        &self.distance
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PenNode {
    Down,
    Up,
}

#[derive(Clone, Debug)]
pub struct RepeatNode {
    count: LexerExpr,
    list: ParserNodeList,
}

impl RepeatNode {
    pub fn new(count: LexerExpr, list: ParserNodeList) -> Self {
        Self { count, list }
    }

    pub fn count(&self) -> &LexerExpr {
        &self.count
    }

    pub fn list(&self) -> &ParserNodeList {
        &self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateNode {
    angle: LexerExpr,
    direction: Direction,
}

impl RotateNode {
    pub fn new(angle: LexerExpr, direction: Direction) -> Self {
        Self { angle, direction }
    }

    pub fn angle(&self) -> &LexerExpr {
        &self.angle
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetHeadingNode {
    angle: LexerExpr,
}

impl SetHeadingNode {
    pub fn new(angle: LexerExpr) -> Self {
        Self { angle }
    }

    pub fn angle(&self) -> &LexerExpr {
        &self.angle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPenColorNode {
    color: LexerExpr,
}

impl SetPenColorNode {
    pub fn new(color: LexerExpr) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &LexerExpr {
        &self.color
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionNode {
    x: Option<LexerExpr>,
    y: Option<LexerExpr>,
}

impl SetPositionNode {
    pub fn new(x: Option<LexerExpr>, y: Option<LexerExpr>) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> Option<&LexerExpr> {
        self.x.as_ref()
    }

    pub fn y(&self) -> Option<&LexerExpr> {
        self.y.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetScreenColorNode {
    color: LexerExpr,
}

impl SetScreenColorNode {
    pub fn new(color: LexerExpr) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &LexerExpr {
        &self.color
    }
}

#[derive(Clone, Debug)]
pub enum ParserNode {
    Assign(AssignNode),
    Call(CallNode),
    Clean,
    ClearScreen,
    Home,
    Let(LetNode),
    Move(MoveNode),
    Pen(PenNode),
    Random(LexerExpr),
    Repeat(RepeatNode),
    Rotate(RotateNode),
    SetHeading(SetHeadingNode),
    SetPenColor(SetPenColorNode),
    SetPosition(SetPositionNode),
    SetScreenColor(SetScreenColorNode),
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
