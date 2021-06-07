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

use super::error::*;
use super::lexer_types::*;

#[derive(Clone, Debug)]
struct LexerState {
    pub list: List,
    pub symbol: String,
    pub number: bool,
}

impl LexerState {
    pub fn new() -> Self {
        Self {
            list: List::new(),
            symbol: String::new(),
            number: false,
        }
    }

    pub fn delimit(&mut self) -> RuntimeResult {
        if !self.symbol.is_empty() {
            let item = if self.number {
                if let Ok(val) = self.symbol.parse::<f64>() {
                    let num = Number::new(val);
                    AnyItem::Number(num)
                } else {
                    let msg = format!("Failed to parse number {}", self.symbol);
                    return Err(RuntimeError::Lexer(msg));
                }
            } else {
                let word = Word::new(&self.symbol);
                AnyItem::Word(word)
            };
            self.list.push(item);
        }

        self.symbol.clear();
        self.number = false;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn go(&mut self, input: &str) -> RuntimeResult<List> {
        let mut iter = input.chars();
        self.lex(&mut iter)
    }

    fn lex(&mut self, iter: &mut Chars) -> RuntimeResult<List> {
        let mut state = LexerState::new();

        while let Some(c) = iter.next() {
            match c {
                '#' => {
                    state.delimit()?;
                    Self::munch(iter);
                }

                '{' => {
                    state.delimit()?;

                    let block = self.lex(iter)?;
                    let item = AnyItem::Block(block);
                    state.list.push(item);
                }

                '}' => {
                    state.delimit()?;
                    break;
                }

                '[' => {
                    state.delimit()?;

                    let inner = self.lex(iter)?;
                    let item = AnyItem::List(inner);
                    state.list.push(item);
                }

                ']' => {
                    state.delimit()?;
                    break;
                }

                '(' => {
                    state.delimit()?;

                    let expr = self.expression(iter)?;
                    let item = AnyItem::Expression(expr);
                    state.list.push(item);
                }

                ')' => {
                    state.delimit()?;
                    break;
                }

                '+' | '-' | '*' | '/' | '=' => {
                    state.delimit()?;

                    let op = Self::operator(c)?;
                    let item = AnyItem::Operator(op);
                    state.list.push(item);
                }

                '.' => {
                    if !state.number {
                        let msg = "Unexpected period".to_string();
                        return Err(RuntimeError::Lexer(msg));
                    }

                    state.symbol.push(c);
                }

                _ => {
                    if c.is_whitespace() {
                        state.delimit()?;
                    } else if c.is_digit(10) {
                        if state.symbol.is_empty() {
                            state.number = true;
                        }
                        state.symbol.push(c);
                    } else if c.is_alphanumeric() {
                        state.symbol.push(c);
                        state.number = false;
                    } else {
                        let msg = format!("Unrecognized character {}", c);
                        return Err(RuntimeError::Lexer(msg));
                    }
                }
            }
        }

        state.delimit()?;

        Ok(state.list)
    }

    fn operator(c: char) -> RuntimeResult<Operator> {
        match c {
            '+' => Ok(Operator::Add),
            '=' => Ok(Operator::Assign),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            _ => {
                let msg = format!("Unrecognized operator {}", c);
                Err(RuntimeError::Lexer(msg))
            }
        }
    }

    fn munch(iter: &mut Chars) {
        for c in iter {
            if c == '\n' || c == '\r' {
                break;
            }
        }
    }

    fn expression(&mut self, iter: &mut Chars) -> RuntimeResult<Expression> {
        fn expr_num_word(item: Option<&AnyItem>) -> RuntimeResult<ExprNumWord> {
            match item {
                Some(AnyItem::Expression(expr)) => Ok(ExprNumWord::Expression(expr.clone())),
                Some(AnyItem::Number(num)) => Ok(ExprNumWord::Number(*num)),
                Some(AnyItem::Word(word)) => Ok(ExprNumWord::Word(word.clone())),
                _ => {
                    let msg = "Expected an expression, number or word".to_string();
                    Err(RuntimeError::Lexer(msg))
                }
            }
        }

        fn op_item(item: Option<&AnyItem>) -> RuntimeResult<Operator> {
            if let Some(AnyItem::Operator(op)) = item {
                Ok(*op)
            } else {
                let msg = "Expected an operator".to_string();
                Err(RuntimeError::Lexer(msg))
            }
        }

        let expr_list = self.lex(iter)?;
        let mut expr_iter = expr_list.iter();

        let a = expr_num_word(expr_iter.next())?;
        let op = op_item(expr_iter.next())?;
        let b = expr_num_word(expr_iter.next())?;

        Ok(Expression::new(a, op, b))
    }
}
