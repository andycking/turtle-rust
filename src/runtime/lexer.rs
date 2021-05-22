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

use std::collections::VecDeque;
use std::ops::Deref;
use std::ops::DerefMut;

use super::data_type::*;
use super::error::*;

struct Stack {
    items: VecDeque<List>,
}

impl Deref for Stack {
    type Target = VecDeque<List>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
}

pub struct Lexer {
    symbol: String,
    attr: WordAttr,
    list: List,
    stack: Stack,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            symbol: String::new(),
            attr: WordAttr::Basic,
            list: List::new(),
            stack: Stack::new(),
        }
    }

    fn reset(&mut self) {
        self.symbol = String::new();
        self.attr = WordAttr::Basic;
    }

    fn has_symbol(&self) -> bool {
        !self.symbol.is_empty()
    }

    fn append_char(&mut self, c: char) {
        self.symbol.push(c);
    }

    fn delimit(&mut self) {
        if !self.symbol.is_empty() {
            let obj = Word::new(&self.symbol, self.attr);
            self.list.push(Box::new(obj));
        }

        self.reset();
    }

    fn attr(&self) -> WordAttr {
        self.attr
    }

    fn set_attr(&mut self, attr: WordAttr) {
        self.attr = attr;
    }

    fn depth(&self) -> usize {
        self.stack.len()
    }

    fn open_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        self.stack.push_front(List::from(items));
    }

    fn close_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        let child = List::from(items);

        let mut parent = self.stack.pop_front().unwrap();
        parent.push(Box::new(child));

        self.list = parent;
    }

    pub fn go(&mut self, input: &str) -> Result<List, InterpreterError> {
        for l in input.lines() {
            let trimmed = l.trim();
            for c in trimmed.chars() {
                match c {
                    ';' => {
                        self.delimit();
                        break;
                    }

                    '[' => {
                        if self.depth() == 32 {
                            return Err(InterpreterError::LexerMaxStack);
                        }

                        self.delimit();
                        self.open_list();
                    }

                    ']' => {
                        if self.depth() == 0 {
                            return Err(InterpreterError::LexerUnbalancedList);
                        }

                        self.delimit();
                        self.close_list();
                    }

                    '(' => {}

                    ')' => {}

                    '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                        self.append_char(c);
                    }

                    '\u{0022}' => {
                        if !self.has_symbol() {
                            return Err(InterpreterError::LexerUnexpectedLiteral);
                        }

                        self.set_attr(WordAttr::Literal);
                    }

                    ':' => {
                        if !self.has_symbol() {
                            return Err(InterpreterError::LexerUnexpectedVariable);
                        }

                        self.set_attr(WordAttr::Variable);
                    }

                    _ => {
                        if c.is_whitespace() {
                            self.delimit();
                        } else if c.is_alphanumeric() {
                            self.append_char(c);
                        } else {
                            return Err(InterpreterError::LexerUnrecognizedCharacter);
                        }
                    }
                }
            }

            self.delimit();
        }

        if self.depth() > 0 {
            return Err(InterpreterError::LexerUnbalancedList);
        }

        Ok(List::from(self.list.consume()))
    }
}
