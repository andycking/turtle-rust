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
use super::error::InterpreterError;
use super::error::InterpreterError::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionTag {
    Home,
    Make,
    Move,
    Pen,
    Procedure,
    Rotate,
    SetPosition,
}

trait Instruction {
    fn tag(&self) -> InstructionTag;
}

#[derive(Clone, Debug, PartialEq)]
struct HomeInstruction {}

impl Instruction for HomeInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Home
    }
}

impl HomeInstruction {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MakeInstruction {
    name: Word,
    value: Word,
}

impl Instruction for MakeInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Make
    }
}

impl MakeInstruction {
    pub fn new(name: Word, value: Word) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MoveDirection {
    Forwards,
    Backwards,
}

#[derive(Clone, Debug, PartialEq)]
struct MoveInstruction {
    distance: Word,
    direction: MoveDirection,
}

impl Instruction for MoveInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Move
    }
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
enum PenOperation {
    Down,
    Up,
    SetColor,
}

#[derive(Clone, Debug, PartialEq)]
struct PenInstruction {
    op: PenOperation,
    color: Option<Word>,
}

impl Instruction for PenInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Pen
    }
}

impl PenInstruction {
    pub fn new(op: PenOperation, color: Option<Word>) -> Self {
        Self { op, color }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ProcInstruction {
    name: Word,
}

impl Instruction for ProcInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Pen
    }
}

impl ProcInstruction {
    pub fn new(name: Word) -> Self {
        Self { name }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RotateDirection {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct RotateInstruction {
    angle: Word,
    direction: RotateDirection,
}

impl Instruction for RotateInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Rotate
    }
}

impl RotateInstruction {
    pub fn new(angle: Word, direction: RotateDirection) -> Self {
        Self { angle, direction }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SetPositionInstruction {
    x: Option<Word>,
    y: Option<Word>,
}

impl Instruction for SetPositionInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::SetPosition
    }
}

impl SetPositionInstruction {
    pub fn new(x: Option<Word>, y: Option<Word>) -> Self {
        Self { x, y }
    }
}

pub type InstructionListItems = Vec<Box<dyn Instruction>>;

struct InstructionList {
    items: InstructionListItems,
}

impl Deref for InstructionList {
    type Target = InstructionListItems;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for InstructionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

struct ListIter<'a> {
    list: &'a List,
    idx: usize,
}

impl<'a> ListIter<'a> {
    pub fn new(list: &'a List) -> Self {
        Self { list, idx: 0 }
    }

    fn is_empty(&self) -> bool {
        self.idx >= self.list.len()
    }

    fn expect(&self, n: usize) -> Result<(), InterpreterError> {
        if self.idx + n >= self.list.len() {
            Err(ParserUnexpectedEnd)
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> &Box<dyn DataType> {
        let temp = self.idx;
        self.idx += 1;
        &self.list[temp]
    }

    fn get_argument(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::Word {
            return Err(ParserExpectedArgument);
        }

        Ok(item.as_any().downcast_ref::<Word>().unwrap())
    }

    fn get_list(&mut self) -> Result<&List, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::List {
            return Err(ParserExpectedList);
        }

        Ok(item.as_any().downcast_ref::<List>().unwrap())
    }

    fn get_procedure(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::Word {
            return Err(ParserExpectedProcedure);
        }

        let word = item.as_any().downcast_ref::<Word>().unwrap();
        if word.attr() == WordAttr::Bare {
            Ok(word)
        } else {
            Err(ParserExpectedProcedure)
        }
    }

    fn get_quoted(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::Word {
            return Err(ParserExpectedProcedure);
        }

        let word = item.as_any().downcast_ref::<Word>().unwrap();
        if word.attr() == WordAttr::Quoted {
            Ok(word)
        } else {
            Err(ParserExpectedQuoted)
        }
    }
}

pub struct Parser<'a> {
    iter: ListIter<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a List) -> Self {
        Self {
            iter: ListIter::new(input),
        }
    }

    pub fn go(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        while !self.iter.is_empty() {
            let proc = self.iter.get_procedure()?;
            match proc.symbol().to_lowercase().as_str() {
                "bk" | "back" => {
                    self.back()?;
                }

                "end" => {
                    self.end()?;
                }

                "fd" | "forward" => {
                    self.forward()?;
                }

                "home" => {
                    self.home()?;
                }

                "lt" | "left" => {
                    self.left()?;
                }

                "make" => {
                    self.make()?;
                }

                "name" => {
                    self.name()?;
                }

                "pd" | "pendown" => {
                    self.pendown()?;
                }

                "pu" | "penup" => {
                    self.penup()?;
                }

                "rt" | "right" => {
                    self.right()?;
                }

                "setpc" | "setpencolor" => {
                    self.setpencolor()?;
                }

                "setpos" => {
                    self.setpos()?;
                }

                "setxy" => {
                    self.setxy()?;
                }

                "setx" => {
                    self.setx()?;
                }

                "sety" => {
                    self.sety()?;
                }

                "to" => {
                    self.to()?;
                }

                _ => {
                    return Err(ParserUnrecognizedProcedure);
                }
            }
        }

        Ok(())
    }

    fn back(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let distance = self.iter.get_argument()?.clone();
        let instr = MoveInstruction::new(distance, MoveDirection::Backwards);
        Ok(())
    }

    fn forward(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let distance = self.iter.get_argument()?.clone();
        let instr = MoveInstruction::new(distance, MoveDirection::Forwards);
        Ok(())
    }

    fn home(&mut self) -> Result<(), InterpreterError> {
        let instr = HomeInstruction::new();
        Ok(())
    }

    fn make(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let name = self.iter.get_quoted()?.clone();
        let value = self.iter.get_argument()?.clone();
        let instr = MakeInstruction::new(name, value);
        Ok(())
    }

    fn name(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2);
        let value = self.iter.get_argument()?.clone();
        let name = self.iter.get_quoted()?.clone();
        let instr = MakeInstruction::new(name, value);
        Ok(())
    }

    fn left(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let angle = self.iter.get_argument()?.clone();
        let instr = RotateInstruction::new(angle, RotateDirection::Left);
        Ok(())
    }

    fn pendown(&mut self) -> Result<(), InterpreterError> {
        let instr = PenInstruction::new(PenOperation::Down, None);
        Ok(())
    }

    fn penup(&mut self) -> Result<(), InterpreterError> {
        let instr = PenInstruction::new(PenOperation::Up, None);
        Ok(())
    }

    fn right(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let angle = self.iter.get_argument()?.clone();
        let instr = RotateInstruction::new(angle, RotateDirection::Right);
        Ok(())
    }

    fn setpencolor(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let color = self.iter.get_argument()?;
        let instr = PenInstruction::new(PenOperation::SetColor, Some(color.clone()));
        Ok(())
    }

    fn setpos(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let pos = self.iter.get_list()?;
        let mut pos_iter = ListIter::new(pos);
        pos_iter.expect(2)?;
        let x = pos_iter.get_argument()?.clone();
        let y = pos_iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(Some(x), Some(y));
        Ok(())
    }

    fn setxy(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let x = self.iter.get_argument()?.clone();
        let y = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(Some(x), Some(y));
        Ok(())
    }

    fn setx(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let x = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(Some(x), None);
        Ok(())
    }

    fn sety(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let y = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(None, Some(y));
        Ok(())
    }

    fn end(&mut self) -> Result<(), InterpreterError> {
        Ok(())
    }

    fn to(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let name = self.iter.get_argument()?.clone();
        let instr = ProcInstruction::new(name);
        Ok(())
    }
}
