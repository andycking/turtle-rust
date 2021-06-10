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
    list: List,
    symbol: String,
    number: bool,
}

impl LexerState {
    pub fn new() -> Self {
        Self {
            list: List::new(),
            symbol: String::new(),
            number: false,
        }
    }

    pub fn delimit(&mut self, idx: usize) -> RuntimeResult {
        if !self.symbol.is_empty() {
            let item = if self.number {
                if let Ok(val) = self.symbol.parse::<f64>() {
                    let num = Number::new(val);
                    AnyItem::Number(num)
                } else {
                    let msg = format!("{}: failed to parse number {}", idx, self.symbol);
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
pub struct Lexer {
    idx: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self { idx: 1 }
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
                    state.delimit(self.idx)?;
                    self.idx += Self::munch(iter);
                }

                '{' => {
                    state.delimit(self.idx)?;

                    let block = self.lex(iter)?;
                    let item = AnyItem::Block(block);
                    state.list.push(item);
                }

                '}' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '[' => {
                    state.delimit(self.idx)?;

                    let inner = self.lex(iter)?;
                    let item = AnyItem::List(inner);
                    state.list.push(item);
                }

                ']' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '(' => {
                    state.delimit(self.idx)?;

                    let bin_expr = self.bin_expr(iter)?;
                    let item = AnyItem::BinExpr(bin_expr);
                    state.list.push(item);
                }

                ')' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '+' | '-' | '*' | '/' | '=' => {
                    state.delimit(self.idx)?;

                    let op = Self::operator(c, self.idx)?;
                    let item = AnyItem::Operator(op);
                    state.list.push(item);
                }

                '.' => {
                    if !state.number {
                        let msg = format!("{}: unexpected period", self.idx);
                        return Err(RuntimeError::Lexer(msg));
                    }

                    state.symbol.push(c);
                }

                _ => {
                    if c.is_whitespace() {
                        state.delimit(self.idx)?;
                    } else if c.is_digit(10) {
                        if state.symbol.is_empty() {
                            state.number = true;
                        }
                        state.symbol.push(c);
                    } else if c.is_alphanumeric() {
                        state.symbol.push(c);
                        state.number = false;
                    } else {
                        let msg = format!("{}: unrecognized character {}", self.idx, c);
                        return Err(RuntimeError::Lexer(msg));
                    }
                }
            }

            self.idx += 1;
        }

        state.delimit(self.idx)?;

        Ok(state.list)
    }

    fn operator(c: char, idx: usize) -> RuntimeResult<Operator> {
        match c {
            '+' => Ok(Operator::Add),
            '=' => Ok(Operator::Assign),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            _ => {
                let msg = format!("{}: unrecognized operator {}", idx, c);
                Err(RuntimeError::Lexer(msg))
            }
        }
    }

    fn munch(iter: &mut Chars) -> usize {
        let mut idx = 0;

        for c in iter {
            if c == '\n' || c == '\r' {
                break;
            }
            idx += 1;
        }

        idx
    }

    fn bin_expr(&mut self, iter: &mut Chars) -> RuntimeResult<BinExpr> {
        fn expression(item: Option<&AnyItem>, idx: usize) -> RuntimeResult<Expression> {
            match item {
                Some(AnyItem::BinExpr(bin_expr)) => Ok(Expression::BinExpr(bin_expr.clone())),
                Some(AnyItem::List(list)) => Ok(Expression::List(list.clone())),
                Some(AnyItem::Number(num)) => Ok(Expression::Number(*num)),
                Some(AnyItem::Word(word)) => Ok(Expression::Word(word.clone())),
                _ => {
                    let msg = format!("{}: expected an BinExpr, number or word", idx);
                    Err(RuntimeError::Lexer(msg))
                }
            }
        }

        fn op_item(item: Option<&AnyItem>, idx: usize) -> RuntimeResult<Operator> {
            if let Some(AnyItem::Operator(op)) = item {
                Ok(*op)
            } else {
                let msg = format!("{}: expected an operator", idx);
                Err(RuntimeError::Lexer(msg))
            }
        }

        let expr_list = self.lex(iter)?;
        let mut expr_iter = expr_list.iter();

        let a = expression(expr_iter.next(), self.idx)?;
        let op = op_item(expr_iter.next(), self.idx)?;
        let b = expression(expr_iter.next(), self.idx)?;

        Ok(BinExpr::new(a, op, b))
    }
}
