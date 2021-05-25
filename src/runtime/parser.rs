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

use std::any::Any;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use super::data_type::*;
use super::error::InterpreterError;
use super::error::InterpreterError::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionTag {
    Call,
    Home,
    MakeVar,
    MakeProc,
    Move,
    Pen,
    Rotate,
    SetPosition,
}

trait Instruction {
    fn tag(&self) -> InstructionTag;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug, PartialEq)]
struct CallInstruction {
    name: Word,
}

impl Instruction for CallInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Call
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CallInstruction {
    pub fn new(name: Word) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HomeInstruction {}

impl Instruction for HomeInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::Home
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HomeInstruction {
    pub fn new() -> Self {
        Self {}
    }
}

type InstructionListItems = Vec<Box<dyn Instruction>>;

struct MakeProcInstruction {
    name: Word,
    instrs: InstructionListItems,
}

impl Instruction for MakeProcInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::MakeProc
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Deref for MakeProcInstruction {
    type Target = InstructionListItems;

    fn deref(&self) -> &Self::Target {
        &self.instrs
    }
}

impl DerefMut for MakeProcInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instrs
    }
}

impl MakeProcInstruction {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            instrs: Vec::new(),
        }
    }

    pub fn consume(&mut self) -> InstructionListItems {
        std::mem::replace(&mut self.instrs, Vec::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MakeVarInstruction {
    name: Word,
    value: Word,
}

impl Instruction for MakeVarInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::MakeVar
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MakeVarInstruction {
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PenInstruction {
    pub fn new(op: PenOperation, color: Option<Word>) -> Self {
        Self { op, color }
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SetPositionInstruction {
    pub fn new(x: Option<Word>, y: Option<Word>) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for dyn Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tag() {
            InstructionTag::Call => {
                let instr = self.as_any().downcast_ref::<CallInstruction>().unwrap();
                write!(f, "call {:?}", instr.name)
            }

            InstructionTag::Home => {
                let instr = self.as_any().downcast_ref::<HomeInstruction>().unwrap();
                write!(f, "home")
            }

            InstructionTag::Move => {
                let instr = self.as_any().downcast_ref::<MoveInstruction>().unwrap();
                write!(f, "move {:?} {:?}", instr.direction, instr.distance)
            }

            InstructionTag::Pen => {
                let instr = self.as_any().downcast_ref::<PenInstruction>().unwrap();
                write!(f, "pen {:?}", instr.op)
            }

            _ => {
                write!(f, "Unkown")
            }
        }
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
    instrs: MakeProcInstruction,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a List) -> Self {
        Self {
            iter: ListIter::new(input),
            instrs: MakeProcInstruction::new(Word::new("main", WordAttr::Bare)),
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
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn end(&mut self) -> Result<(), InterpreterError> {
        Ok(())
    }

    fn forward(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let distance = self.iter.get_argument()?.clone();
        let instr = MoveInstruction::new(distance, MoveDirection::Forwards);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn home(&mut self) -> Result<(), InterpreterError> {
        let instr = HomeInstruction::new();
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn left(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let angle = self.iter.get_argument()?.clone();
        let instr = RotateInstruction::new(angle, RotateDirection::Left);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn make(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let name = self.iter.get_quoted()?.clone();
        let value = self.iter.get_argument()?.clone();
        let instr = MakeVarInstruction::new(name, value);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn name(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let value = self.iter.get_argument()?.clone();
        let name = self.iter.get_quoted()?.clone();
        let instr = MakeVarInstruction::new(name, value);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn pendown(&mut self) -> Result<(), InterpreterError> {
        let instr = PenInstruction::new(PenOperation::Down, None);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn penup(&mut self) -> Result<(), InterpreterError> {
        let instr = PenInstruction::new(PenOperation::Up, None);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn right(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let angle = self.iter.get_argument()?.clone();
        let instr = RotateInstruction::new(angle, RotateDirection::Right);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn setpencolor(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let color = self.iter.get_argument()?;
        let instr = PenInstruction::new(PenOperation::SetColor, Some(color.clone()));
        self.instrs.push(Box::new(instr));
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
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn setxy(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;
        let x = self.iter.get_argument()?.clone();
        let y = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(Some(x), Some(y));
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn setx(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let x = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(Some(x), None);
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn sety(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let y = self.iter.get_argument()?.clone();
        let instr = SetPositionInstruction::new(None, Some(y));
        self.instrs.push(Box::new(instr));
        Ok(())
    }

    fn to(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;
        let name = self.iter.get_argument()?.clone();
        let instr = MakeProcInstruction::new(name);
        // Hrm.
        Ok(())
    }
}
