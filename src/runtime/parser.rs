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
use super::instr::*;

#[derive(Clone, Debug)]
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

    fn next(&mut self) -> DataType {
        let temp = self.idx;
        self.idx += 1;
        self.list[temp].clone()
    }

    fn get_argument(&mut self) -> Result<Word, InterpreterError> {
        if let DataType::Word(word) = self.next() {
            Ok(word)
        } else {
            Err(ParserExpectedArgument)
        }
    }

    fn get_list(&mut self) -> Result<List, InterpreterError> {
        if let DataType::List(list) = self.next() {
            Ok(list)
        } else {
            Err(ParserExpectedList)
        }
    }

    fn get_procedure(&mut self) -> Result<Word, InterpreterError> {
        if let DataType::Word(word) = self.next() {
            if word.attr() == WordAttr::Bare {
                Ok(word)
            } else {
                Err(ParserExpectedProcedure)
            }
        } else {
            Err(ParserExpectedProcedure)
        }
    }

    fn get_quoted(&mut self) -> Result<Word, InterpreterError> {
        if let DataType::Word(word) = self.next() {
            if word.attr() == WordAttr::Quoted {
                Ok(word)
            } else {
                Err(ParserExpectedQuoted)
            }
        } else {
            Err(ParserExpectedQuoted)
        }
    }
}

pub struct Parser<'a> {
    iter: ListIter<'a>,
    list: MakeProcInstruction,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a List) -> Self {
        Self {
            iter: ListIter::new(input),
            list: MakeProcInstruction::new(Word::new("main", WordAttr::Bare)),
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

        let distance = self.iter.get_argument()?;
        let move_instr = MoveInstruction::new(distance, MoveDirection::Backwards);
        let instr = Instruction::Move(move_instr);
        self.list.push(instr);

        Ok(())
    }

    fn end(&mut self) -> Result<(), InterpreterError> {
        Ok(())
    }

    fn forward(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let distance = self.iter.get_argument()?;
        let move_instr = MoveInstruction::new(distance, MoveDirection::Forwards);
        let instr = Instruction::Move(move_instr);
        self.list.push(instr);

        Ok(())
    }

    fn home(&mut self) -> Result<(), InterpreterError> {
        let home_instr = HomeInstruction {};
        let instr = Instruction::Home(home_instr);
        self.list.push(instr);

        Ok(())
    }

    fn left(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let angle = self.iter.get_argument()?;
        let rotate_instr = RotateInstruction::new(angle, RotateDirection::Left);
        let instr = Instruction::Rotate(rotate_instr);
        self.list.push(instr);

        Ok(())
    }

    fn make(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;

        let name = self.iter.get_quoted()?;
        let value = self.iter.get_argument()?;
        let make_instr = MakeVarInstruction::new(name, value);
        let instr = Instruction::MakeVar(make_instr);
        self.list.push(instr);

        Ok(())
    }

    fn name(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;

        let value = self.iter.get_argument()?;
        let name = self.iter.get_quoted()?;
        let make_instr = MakeVarInstruction::new(name, value);
        let instr = Instruction::MakeVar(make_instr);
        self.list.push(instr);

        Ok(())
    }

    fn pendown(&mut self) -> Result<(), InterpreterError> {
        let pen_instr = PenInstruction::new(PenOperation::Down, None);
        let instr = Instruction::Pen(pen_instr);
        self.list.push(instr);

        Ok(())
    }

    fn penup(&mut self) -> Result<(), InterpreterError> {
        let pen_instr = PenInstruction::new(PenOperation::Up, None);
        let instr = Instruction::Pen(pen_instr);
        self.list.push(instr);

        Ok(())
    }

    fn right(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let angle = self.iter.get_argument()?;
        let rotate_instr = RotateInstruction::new(angle, RotateDirection::Right);
        let instr = Instruction::Rotate(rotate_instr);
        self.list.push(instr);

        Ok(())
    }

    fn setpencolor(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let color = self.iter.get_argument()?;
        let pen_instr = PenInstruction::new(PenOperation::SetColor, Some(color));
        let instr = Instruction::Pen(pen_instr);
        self.list.push(instr);

        Ok(())
    }

    fn setpos(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let pos = self.iter.get_list()?;
        let mut pos_iter = ListIter::new(&pos);

        pos_iter.expect(2)?;
        let x = pos_iter.get_argument()?;
        let y = pos_iter.get_argument()?;
        let pos_instr = SetPositionInstruction::new(Some(x), Some(y));
        let instr = Instruction::SetPosition(pos_instr);
        self.list.push(instr);

        Ok(())
    }

    fn setxy(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(2)?;

        let x = self.iter.get_argument()?;
        let y = self.iter.get_argument()?;
        let pos_instr = SetPositionInstruction::new(Some(x), Some(y));
        let instr = Instruction::SetPosition(pos_instr);
        self.list.push(instr);

        Ok(())
    }

    fn setx(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let x = self.iter.get_argument()?;
        let pos_instr = SetPositionInstruction::new(Some(x), None);
        let instr = Instruction::SetPosition(pos_instr);
        self.list.push(instr);

        Ok(())
    }

    fn sety(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let y = self.iter.get_argument()?;
        let pos_instr = SetPositionInstruction::new(None, Some(y));
        let instr = Instruction::SetPosition(pos_instr);
        self.list.push(instr);

        Ok(())
    }

    fn to(&mut self) -> Result<(), InterpreterError> {
        self.iter.expect(1)?;

        let name = self.iter.get_argument()?;
        let make_instr = MakeProcInstruction::new(name);
        let instr = Instruction::MakeProc(make_instr);

        Ok(())
    }
}
