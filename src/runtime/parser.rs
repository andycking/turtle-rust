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

use super::data_type::*;
use super::error::InterpreterError;
use super::error::InterpreterError::*;
use super::instr::*;

#[derive(Clone, Debug)]
struct ListIter<'a> {
    list: &'a [DataType],
    idx: usize,
}

impl<'a> ListIter<'a> {
    pub fn new(list: &'a [DataType]) -> Self {
        Self { list, idx: 0 }
    }

    fn is_empty(&self) -> bool {
        self.idx >= self.list.len()
    }

    fn expect(&self, n: usize) -> Result<(), InterpreterError> {
        if self.idx + n > self.list.len() {
            Err(ParserUnexpectedEndOfInput)
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> DataType {
        let temp = self.idx;
        self.idx += 1;
        self.list[temp].clone()
    }

    fn slice_to(&mut self, name: &str) -> Result<&'a [DataType], InterpreterError> {
        for i in self.idx..self.list.len() {
            if let DataType::Word(word) = &self.list[i] {
                if word.symbol() == name {
                    let slice = &self.list[self.idx..i];
                    self.idx = i + 1;
                    return Ok(slice);
                }
            }
        }

        Err(ParserExpectedEndOfProcedure)
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

    fn get_quoted_word(&mut self) -> Result<Word, InterpreterError> {
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

    fn get_word(&mut self) -> Result<Word, InterpreterError> {
        if let DataType::Word(word) = self.next() {
            Ok(word)
        } else {
            Err(ParserExpectedArgument)
        }
    }
}

pub struct Parser {
    procedures: HashMap<String, usize>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            procedures: HashMap::new(),
        }
    }

    pub fn go(&mut self, input: &[DataType]) -> Result<InstructionList, InterpreterError> {
        let mut iter = ListIter::new(input);
        let mut list = InstructionList::new();

        iter.expect(1)?;

        while !iter.is_empty() {
            let proc = iter.get_procedure()?;
            let symbol = proc.symbol();
            match symbol.to_lowercase().as_str() {
                "bk" | "back" => {
                    let instr = self.back(&mut iter)?;
                    list.push(instr);
                }

                "fd" | "forward" => {
                    let instr = self.forward(&mut iter)?;
                    list.push(instr);
                }

                "home" => {
                    let instr = self.home(&mut iter);
                    list.push(instr);
                }

                "lt" | "left" => {
                    let instr = self.left(&mut iter)?;
                    list.push(instr);
                }

                "make" => {
                    let instr = self.make(&mut iter)?;
                    list.push(instr);
                }

                "name" => {
                    let instr = self.name(&mut iter)?;
                    list.push(instr);
                }

                "pd" | "pendown" => {
                    let instr = self.pendown(&mut iter);
                    list.push(instr);
                }

                "pu" | "penup" => {
                    let instr = self.penup(&mut iter);
                    list.push(instr);
                }

                "repeat" => {
                    let instr = self.repeat(&mut iter)?;
                    list.push(instr);
                }

                "rt" | "right" => {
                    let instr = self.right(&mut iter)?;
                    list.push(instr);
                }

                "setpc" | "setpencolor" => {
                    let instr = self.setpencolor(&mut iter)?;
                    list.push(instr);
                }

                "setpos" => {
                    let instr = self.setpos(&mut iter)?;
                    list.push(instr);
                }

                "setxy" => {
                    let instr = self.setxy(&mut iter)?;
                    list.push(instr);
                }

                "setx" => {
                    let instr = self.setx(&mut iter)?;
                    list.push(instr);
                }

                "sety" => {
                    let instr = self.sety(&mut iter)?;
                    list.push(instr);
                }

                "to" => {
                    let instr = self.to(&mut iter)?;
                    list.push(instr);
                }

                _ => {
                    if self.procedures.contains_key(symbol) {
                        let num_args = self.procedures[symbol];
                        let instr = self.call(&mut iter, proc, num_args)?;
                        list.push(instr);
                    } else {
                        return Err(ParserUnrecognizedProcedure);
                    }
                }
            }
        }

        println!("{:?}", list);

        Ok(list)
    }

    fn back(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let distance = iter.get_word()?;
        let move_instr = MoveInstruction::new(distance, MoveDirection::Backwards);
        let instr = Instruction::Move(move_instr);

        Ok(instr)
    }

    fn call(
        &mut self,
        iter: &mut ListIter,
        name: Word,
        num_args: usize,
    ) -> Result<Instruction, InterpreterError> {
        iter.expect(num_args)?;

        let mut list = List::new();
        for _ in 0..num_args {
            list.push(iter.next());
        }
        let call_instr = CallInstruction::new(name, list);
        let instr = Instruction::Call(call_instr);

        Ok(instr)
    }

    fn forward(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let distance = iter.get_word()?;
        let move_instr = MoveInstruction::new(distance, MoveDirection::Forwards);
        let instr = Instruction::Move(move_instr);

        Ok(instr)
    }

    fn home(&mut self, _: &mut ListIter) -> Instruction {
        let home_instr = HomeInstruction {};
        Instruction::Home(home_instr)
    }

    fn left(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let angle = iter.get_word()?;
        let rotate_instr = RotateInstruction::new(angle, RotateDirection::Left);
        let instr = Instruction::Rotate(rotate_instr);

        Ok(instr)
    }

    fn make(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(2)?;

        let name = iter.get_quoted_word()?;
        let value = iter.get_word()?;
        let make_instr = MakeVarInstruction::new(name, value);
        let instr = Instruction::MakeVar(make_instr);

        Ok(instr)
    }

    fn name(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(2)?;

        let value = iter.get_word()?;
        let name = iter.get_quoted_word()?;
        let make_instr = MakeVarInstruction::new(name, value);
        let instr = Instruction::MakeVar(make_instr);

        Ok(instr)
    }

    fn pendown(&mut self, _: &mut ListIter) -> Instruction {
        let pen_instr = PenInstruction::new(PenOperation::Down, None);
        Instruction::Pen(pen_instr)
    }

    fn penup(&mut self, _: &mut ListIter) -> Instruction {
        let pen_instr = PenInstruction::new(PenOperation::Up, None);
        Instruction::Pen(pen_instr)
    }

    fn repeat(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(2)?;

        let count = iter.get_word()?;
        let list = iter.get_list()?;
        let instr_list = self.go(&list[..])?;
        let rep_instr = RepeatInstruction::new(count, instr_list);
        let instr = Instruction::Repeat(rep_instr);

        Ok(instr)
    }

    fn right(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let angle = iter.get_word()?;
        let rotate_instr = RotateInstruction::new(angle, RotateDirection::Right);
        let instr = Instruction::Rotate(rotate_instr);

        Ok(instr)
    }

    fn setpencolor(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let color = iter.get_word()?;
        let pen_instr = PenInstruction::new(PenOperation::SetColor, Some(color));
        let instr = Instruction::Pen(pen_instr);

        Ok(instr)
    }

    fn setpos(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let pos = iter.get_list()?;
        let mut pos_iter = ListIter::new(&pos);

        pos_iter.expect(2)?;
        let x = pos_iter.get_word()?;
        let y = pos_iter.get_word()?;
        let pos_instr = SetPositionInstruction::new(Some(x), Some(y));
        let instr = Instruction::SetPosition(pos_instr);

        Ok(instr)
    }

    fn setxy(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(2)?;

        let x = iter.get_word()?;
        let y = iter.get_word()?;
        let pos_instr = SetPositionInstruction::new(Some(x), Some(y));
        let instr = Instruction::SetPosition(pos_instr);

        Ok(instr)
    }

    fn setx(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let x = iter.get_word()?;
        let pos_instr = SetPositionInstruction::new(Some(x), None);
        let instr = Instruction::SetPosition(pos_instr);

        Ok(instr)
    }

    fn sety(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let y = iter.get_word()?;
        let pos_instr = SetPositionInstruction::new(None, Some(y));
        let instr = Instruction::SetPosition(pos_instr);

        Ok(instr)
    }

    fn to(&mut self, iter: &mut ListIter) -> Result<Instruction, InterpreterError> {
        iter.expect(1)?;

        let name = iter.get_word()?;
        self.check_dupe(name.symbol(), 0)?;
        let slice = iter.slice_to("end")?;
        let instr_list = self.go(slice)?;
        let make_instr = MakeProcInstruction::new(name, 0, instr_list);
        let instr = Instruction::MakeProc(make_instr);

        Ok(instr)
    }

    fn check_dupe(&mut self, name: &str, num_args: usize) -> Result<(), InterpreterError> {
        if self.procedures.insert(name.to_string(), num_args).is_none() {
            Ok(())
        } else {
            Err(ParserDuplicateProcedure)
        }
    }
}
