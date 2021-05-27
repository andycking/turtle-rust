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

use std::ops::Deref;
use std::ops::DerefMut;

use super::data_type::*;

#[derive(Clone, Debug)]
pub struct CallInstruction {
    name: Word,
    list: List,
}

impl CallInstruction {
    pub fn new(name: Word, list: List) -> Self {
        Self { name, list }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HomeInstruction {}

#[derive(Clone, Debug)]
pub struct MakeProcInstruction {
    name: Word,
    num_args: usize,
    list: InstructionList,
}

impl MakeProcInstruction {
    pub fn new(name: Word, num_args: usize, list: InstructionList) -> Self {
        Self {
            name,
            num_args,
            list,
        }
    }
}

impl Deref for MakeProcInstruction {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for MakeProcInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MakeVarInstruction {
    name: Word,
    value: Word,
}

impl MakeVarInstruction {
    pub fn new(name: Word, value: Word) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveDirection {
    Forwards,
    Backwards,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MoveInstruction {
    distance: Word,
    direction: MoveDirection,
}

impl MoveInstruction {
    pub fn new(distance: Word, direction: MoveDirection) -> Self {
        Self {
            distance,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PenOperation {
    Down,
    Up,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PenInstruction {
    op: PenOperation,
}

impl PenInstruction {
    pub fn new(op: PenOperation) -> Self {
        Self { op }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PenColorInstruction {
    color: Word,
}

impl PenColorInstruction {
    pub fn new(color: Word) -> Self {
        Self { color }
    }
}

#[derive(Clone, Debug)]
pub struct RepeatInstruction {
    count: Word,
    list: InstructionList,
}

impl RepeatInstruction {
    pub fn new(count: Word, list: InstructionList) -> Self {
        Self { count, list }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RotateDirection {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotateInstruction {
    angle: Word,
    direction: RotateDirection,
}

impl RotateInstruction {
    pub fn new(angle: Word, direction: RotateDirection) -> Self {
        Self { angle, direction }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenColorInstruction {
    color: Word,
}

impl ScreenColorInstruction {
    pub fn new(color: Word) -> Self {
        Self { color }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionInstruction {
    x: Option<Word>,
    y: Option<Word>,
}

impl SetPositionInstruction {
    pub fn new(x: Option<Word>, y: Option<Word>) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Call(CallInstruction),
    Home(HomeInstruction),
    MakeProc(MakeProcInstruction),
    MakeVar(MakeVarInstruction),
    Move(MoveInstruction),
    Pen(PenInstruction),
    PenColor(PenColorInstruction),
    Repeat(RepeatInstruction),
    Rotate(RotateInstruction),
    ScreenColor(ScreenColorInstruction),
    SetPosition(SetPositionInstruction),
}

#[derive(Clone, Debug)]
pub struct InstructionList {
    list: Vec<Instruction>,
}

impl Deref for InstructionList {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for InstructionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl InstructionList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}
