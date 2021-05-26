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

use std::str::Chars;

use super::data_type::*;
use super::error::*;

#[derive(Clone, Copy, Debug)]
pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn go(&mut self, input: &str) -> Result<List, InterpreterError> {
        let mut iter = input.chars();
        self.lex(&mut iter)
    }

    fn lex(&mut self, iter: &mut Chars) -> Result<List, InterpreterError> {
        let mut list = List::new();
        let mut symbol = String::new();
        let mut attr = WordAttr::Bare;

        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    if !symbol.is_empty() {
                        let word = Word::new(&symbol, attr);
                        let dt = DataType::Word(word);
                        list.push(dt);

                        symbol = String::new();
                        attr = WordAttr::Bare;
                    }

                    let child = self.lex(iter)?;
                    let dt = DataType::List(child);
                    list.push(dt);
                }

                ']' => {
                    if !symbol.is_empty() {
                        let word = Word::new(&symbol, attr);
                        let dt = DataType::Word(word);
                        list.push(dt);

                        symbol = String::new();
                        attr = WordAttr::Bare;
                    }

                    break;
                }

                '(' => {}

                ')' => {}

                '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                    symbol.push(c);
                }

                '\u{0022}' => {
                    if !symbol.is_empty() {
                        return Err(InterpreterError::LexerUnexpectedQuote);
                    }
                    attr = WordAttr::Quoted;
                }

                ':' => {
                    if !symbol.is_empty() {
                        return Err(InterpreterError::LexerUnexpectedValueOf);
                    }
                    attr = WordAttr::ValueOf;
                }

                _ => {
                    if c.is_whitespace() {
                        if !symbol.is_empty() {
                            let word = Word::new(&symbol, attr);
                            let dt = DataType::Word(word);
                            list.push(dt);

                            symbol = String::new();
                            attr = WordAttr::Bare;
                        }
                    } else if c.is_alphanumeric() {
                        symbol.push(c);
                    } else {
                        return Err(InterpreterError::LexerUnrecognizedCharacter);
                    }
                }
            }
        }

        if !symbol.is_empty() {
            let word = Word::new(&symbol, attr);
            let dt = DataType::Word(word);
            list.push(dt);
        }

        println!("{:?}", list);

        Ok(list)
    }
}
