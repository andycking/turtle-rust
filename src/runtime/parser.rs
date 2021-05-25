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

use super::data_type::*;
use super::error::InterpreterError;
use super::error::InterpreterError::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionTag {
    Move,
    Pen,
    Rotate,
    SetPosition,
}

trait Instruction {
    fn tag(&self) -> InstructionTag;
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
    x: Option<f64>,
    y: Option<f64>,
}

impl Instruction for SetPositionInstruction {
    fn tag(&self) -> InstructionTag {
        InstructionTag::SetPosition
    }
}

impl SetPositionInstruction {
    pub fn new(x: Option<f64>, y: Option<f64>) -> Self {
        Self { x, y }
    }
}

pub struct Parser<'a> {
    input: &'a List,
    idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a List) -> Self {
        Self { input, idx: 0 }
    }

    pub fn go(&mut self) -> Result<(), InterpreterError> {
        if self.input.is_empty() {
            return Err(ParserNoInput);
        }

        while self.idx < self.input.len() {
            let proc = self.get_procedure()?;
            match proc.symbol().to_lowercase().as_str() {
                "bk" | "back" => {
                    self.back()?;
                }

                "fd" | "forward" => {
                    self.forward()?;
                }

                "home" => {}

                "lt" | "left" => {
                    self.left()?;
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

                "setxy" => {}

                "setx" => {}

                "sety" => {}

                _ => {
                    return Err(ParserUnrecognizedProcedure);
                }
            }
        }

        Ok(())
    }

    fn back(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let distance = self.get_argument()?;
        let instruction = MoveInstruction::new(distance.clone(), MoveDirection::Backwards);
        Ok(())
    }

    fn forward(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let distance = self.get_argument()?;
        let instruction = MoveInstruction::new(distance.clone(), MoveDirection::Forwards);
        Ok(())
    }

    fn left(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let angle = self.get_argument()?;
        let instruction = RotateInstruction::new(angle.clone(), RotateDirection::Left);
        Ok(())
    }

    fn pendown(&mut self) -> Result<(), InterpreterError> {
        let instruction = PenInstruction::new(PenOperation::Down, None);
        Ok(())
    }

    fn penup(&mut self) -> Result<(), InterpreterError> {
        let instruction = PenInstruction::new(PenOperation::Up, None);
        Ok(())
    }

    fn right(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let angle = self.get_argument()?;
        let instruction = RotateInstruction::new(angle.clone(), RotateDirection::Right);
        Ok(())
    }

    fn setpencolor(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let color = self.get_argument()?;
        let instruction = PenInstruction::new(PenOperation::SetColor, Some(color.clone()));
        Ok(())
    }

    fn setpos(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        let pos = self.get_list(2)?;
        Ok(())
    }

    fn expect(&self, n: usize) -> Result<(), InterpreterError> {
        if self.idx + n >= self.input.len() {
            Err(ParserUnexpectedEnd)
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> &Box<dyn DataType> {
        let temp = self.idx;
        self.idx += 1;
        &self.input[temp]
    }

    fn get_procedure(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::Word {
            return Err(ParserExpectedProcedure);
        }

        let word = item.as_any().downcast_ref::<Word>().unwrap();
        if word.attr() == WordAttr::Basic {
            Ok(word)
        } else {
            Err(ParserExpectedProcedure)
        }
    }

    fn get_argument(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::Word {
            return Err(ParserExpectedArgument);
        }

        Ok(item.as_any().downcast_ref::<Word>().unwrap())
    }

    fn get_list(&mut self, n: usize) -> Result<&List, InterpreterError> {
        let item = self.next();

        if item.tag() != DataTypeTag::List {
            return Err(ParserExpectedList);
        }

        let list = item.as_any().downcast_ref::<List>().unwrap();
        if list.len() != n {
            return Err(ParserTooFewItemsInList);
        }

        Ok(list)
    }
}
