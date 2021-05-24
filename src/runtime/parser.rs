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
use super::error::*;

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
            return Err(InterpreterError::ParserNoInput);
        }

        while self.idx < self.input.len() {
            let cmd = self.get_command()?;
            match cmd.symbol().to_lowercase().as_str() {
                "bk" | "back" => {
                    self.back()?;
                }

                "fd" | "forward" => {}

                "home" => {}

                "lt" | "left" => {}

                "pd" | "pendown" => {}

                "pu" | "penup" => {}

                "rt" | "right" => {}

                "setpc" | "setpencolor" => {}

                "setpos" => {}

                "setxy" => {}

                "setx" => {}

                "sety" => {}

                _ => {
                    return Err(InterpreterError::ParserUnrecognizedCommand);
                }
            }
        }

        Ok(())
    }

    fn back(&mut self) -> Result<(), InterpreterError> {
        self.expect(1)?;
        self.get_distance()?;
        Ok(())
    }

    fn expect(&self, n: usize) -> Result<(), InterpreterError> {
        if self.idx + n >= self.input.len() {
            Err(InterpreterError::ParserUnexpectedEnd)
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> &Box<dyn DataType> {
        let temp = self.idx;
        self.idx += 1;
        &self.input[temp]
    }

    fn get_distance(&mut self) -> Result<f64, InterpreterError> {
        let item = self.next();

        match item.symbol().parse::<f64>() {
            Ok(distance) => Ok(distance),
            _ => Err(InterpreterError::ParserExpectedDistance),
        }
    }

    fn get_command(&mut self) -> Result<&Word, InterpreterError> {
        let item = self.next();

        if item.tag() != Tag::Word {
            return Err(InterpreterError::ParserExpectedCommand);
        }

        let word = item.as_any().downcast_ref::<Word>().unwrap();
        if word.attr() == WordAttr::Basic {
            Ok(word)
        } else {
            Err(InterpreterError::ParserExpectedCommand)
        }
    }
}
