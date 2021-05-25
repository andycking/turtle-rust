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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    symbol: String,
    attr: WordAttr,
    list: List,
    stack: Stack,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            symbol: String::new(),
            attr: WordAttr::Bare,
            list: List::new(),
            stack: Stack::new(),
        }
    }

    fn has_symbol(&self) -> bool {
        !self.symbol.is_empty()
    }

    fn append_char(&mut self, c: char) {
        self.symbol.push(c);
    }

    fn delimit(&mut self) {
        if !self.symbol.is_empty() {
            let word = Word::new(&self.symbol, self.attr);
            let data_type = DataType::Word(word);
            self.list.push(data_type);

            self.symbol = String::new();
        }

        self.attr = WordAttr::Bare;
    }

    fn set_attr(&mut self, attr: WordAttr) {
        self.attr = attr;
    }

    fn depth(&self) -> usize {
        self.stack.len()
    }

    fn open_list(&mut self) {
        self.symbol = String::new();
        self.attr = WordAttr::Bare;

        let items = self.list.consume();
        self.stack.push_front(List::from(items));
    }

    fn close_list(&mut self) {
        self.symbol = String::new();
        self.attr = WordAttr::Bare;

        let items = self.list.consume();
        let child = List::from(items);
        let data_type = DataType::List(child);

        let mut parent = self.stack.pop_front().unwrap();
        parent.push(data_type);

        self.list = parent;
    }

    pub fn go(&mut self) -> Result<List, InterpreterError> {
        if self.input.is_empty() {
            return Err(InterpreterError::LexerNoInput);
        }

        for l in self.input.lines() {
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
                            return Err(InterpreterError::LexerUnexpectedQuote);
                        }

                        self.set_attr(WordAttr::Quoted);
                    }

                    ':' => {
                        if !self.has_symbol() {
                            return Err(InterpreterError::LexerUnexpectedValueOf);
                        }

                        self.set_attr(WordAttr::ValueOf);
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
