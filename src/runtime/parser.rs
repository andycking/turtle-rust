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

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn go(&mut self, input: List) -> Result<(), InterpreterError> {
        let mut iter = input.iter().peekable();

        while let Some(item) = iter.next() {
            if item.tag() != Tag::Word {
                return Err(InterpreterError::ParserExpectedCommand);
            }

            let word = item.as_any().downcast_ref::<Word>().unwrap();
            if word.attr() != WordAttr::Basic {
                return Err(InterpreterError::ParserExpectedCommand);
            }

            match item.symbol().to_lowercase().as_str() {
                "fd" | "forward" => {}

                _ => {}
            }
        }

        Ok(())
    }

    fn is_command(&self, item: &Box<dyn DataType>) -> Result<(), InterpreterError> {
        Ok(())
    }
}
