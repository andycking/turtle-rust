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
    val: ExprNumWord,
}

impl AssignNode {
    pub fn new(name: String, val: ExprNumWord) -> Self {
        Self { name, val }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn val(&self) -> &ExprNumWord {
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
    distance: ExprNumWord,
    direction: Direction,
}

impl MoveNode {
    pub fn new(distance: ExprNumWord, direction: Direction) -> Self {
        Self {
            distance,
            direction,
        }
    }

    pub fn distance(&self) -> &ExprNumWord {
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
    count: ExprNumWord,
    list: NodeList,
}

impl RepeatNode {
    pub fn new(count: ExprNumWord, list: NodeList) -> Self {
        Self { count, list }
    }

    pub fn count(&self) -> &ExprNumWord {
        &self.count
    }

    pub fn list(&self) -> &NodeList {
        &self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateNode {
    angle: ExprNumWord,
    direction: Direction,
}

impl RotateNode {
    pub fn new(angle: ExprNumWord, direction: Direction) -> Self {
        Self { angle, direction }
    }

    pub fn angle(&self) -> &ExprNumWord {
        &self.angle
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetHeadingNode {
    angle: ExprNumWord,
}

impl SetHeadingNode {
    pub fn new(angle: ExprNumWord) -> Self {
        Self { angle }
    }

    pub fn angle(&self) -> &ExprNumWord {
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
    x: Option<ExprNumWord>,
    y: Option<ExprNumWord>,
}

impl SetPositionNode {
    pub fn new(x: Option<ExprNumWord>, y: Option<ExprNumWord>) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> Option<&ExprNumWord> {
        self.x.as_ref()
    }

    pub fn y(&self) -> Option<&ExprNumWord> {
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

pub type FuncDefinition = NodeList;

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
