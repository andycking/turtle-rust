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
    val: Expression,
}

impl AssignNode {
    pub fn new(name: String, val: Expression) -> Self {
        Self { name, val }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn val(&self) -> &Expression {
        &self.val
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallNode {
    name: Word,
}

impl CallNode {
    pub fn new(name: Word) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &Word {
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

pub type LetNode = AssignNode;

#[derive(Clone, Debug, PartialEq)]
pub struct MoveNode {
    distance: Expression,
    direction: Direction,
}

impl MoveNode {
    pub fn new(distance: Expression, direction: Direction) -> Self {
        Self {
            distance,
            direction,
        }
    }

    pub fn distance(&self) -> &Expression {
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
    count: Expression,
    list: NodeList,
}

impl RepeatNode {
    pub fn new(count: Expression, list: NodeList) -> Self {
        Self { count, list }
    }

    pub fn count(&self) -> &Expression {
        &self.count
    }

    pub fn list(&self) -> &NodeList {
        &self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateNode {
    angle: Expression,
    direction: Direction,
}

impl RotateNode {
    pub fn new(angle: Expression, direction: Direction) -> Self {
        Self { angle, direction }
    }

    pub fn angle(&self) -> &Expression {
        &self.angle
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetHeadingNode {
    angle: Expression,
}

impl SetHeadingNode {
    pub fn new(angle: Expression) -> Self {
        Self { angle }
    }

    pub fn angle(&self) -> &Expression {
        &self.angle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPenColorNode {
    color: ListNumWord,
}

impl SetPenColorNode {
    pub fn new(color: ListNumWord) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &ListNumWord {
        &self.color
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionNode {
    x: Option<Expression>,
    y: Option<Expression>,
}

impl SetPositionNode {
    pub fn new(x: Option<Expression>, y: Option<Expression>) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> Option<&Expression> {
        self.x.as_ref()
    }

    pub fn y(&self) -> Option<&Expression> {
        self.y.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetScreenColorNode {
    color: ListNumWord,
}

impl SetScreenColorNode {
    pub fn new(color: ListNumWord) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &ListNumWord {
        &self.color
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Assign(AssignNode),
    Call(CallNode),
    Clean,
    ClearScreen,
    Home,
    Let(LetNode),
    Move(MoveNode),
    Pen(PenNode),
    Repeat(RepeatNode),
    Rotate(RotateNode),
    SetHeading(SetHeadingNode),
    SetPenColor(SetPenColorNode),
    SetPosition(SetPositionNode),
    SetScreenColor(SetScreenColorNode),
}

pub type NodeList = Vec<Node>;

#[derive(Clone, Debug)]
pub struct FuncDefinition {
    builtin: bool,
    num_args: u8,
    pub list: NodeList,
}

impl FuncDefinition {
    pub fn new(builtin: bool, num_args: u8, list: NodeList) -> Self {
        Self {
            builtin,
            num_args,
            list,
        }
    }
}

pub type FuncMap = HashMap<String, FuncDefinition>;

#[derive(Clone, Debug)]
pub struct ParserOutput {
    pub list: NodeList,
    pub fmap: FuncMap,
}

impl ParserOutput {
    pub fn new(list: NodeList, fmap: FuncMap) -> Self {
        Self { list, fmap }
    }
}
