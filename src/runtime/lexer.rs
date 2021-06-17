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

use std::iter::Peekable;
use std::str::Chars;

use super::error::*;
use super::lexer_types::*;

#[derive(Clone, Debug)]
struct LexerState {
    list: LexerList,
    symbol: String,
    number: bool,
}

impl LexerState {
    pub fn new() -> Self {
        Self {
            list: LexerList::new(),
            symbol: String::new(),
            number: false,
        }
    }

    pub fn delimit(&mut self, idx: usize) -> RuntimeResult {
        if !self.symbol.is_empty() {
            let item = if self.number {
                if let Ok(val) = self.symbol.parse::<f64>() {
                    LexerAny::LexerNumber(val)
                } else {
                    let msg = format!("{}: failed to parse number \"{}\"", idx, self.symbol);
                    return Err(RuntimeError::Lexer(msg));
                }
            } else {
                LexerAny::LexerWord(self.symbol.to_string())
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

    pub fn go(&mut self, input: &str) -> RuntimeResult<LexerList> {
        let mut iter = input.chars().peekable();
        self.lex(&mut iter)
    }

    fn lex(&mut self, iter: &mut Peekable<Chars>) -> RuntimeResult<LexerList> {
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
                    let item = LexerAny::LexerBlock(block);
                    state.list.push(item);
                }

                '}' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '[' => {
                    state.delimit(self.idx)?;

                    let inner = self.lex(iter)?;
                    let item = LexerAny::LexerList(inner);
                    state.list.push(item);
                }

                ']' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '(' => {
                    state.delimit(self.idx)?;

                    let bin_expr = self.get_bin_expr(iter)?;
                    let item = LexerAny::LexerBinExpr(bin_expr);
                    state.list.push(item);
                }

                ')' => {
                    state.delimit(self.idx)?;
                    break;
                }

                '-' => {
                    state.delimit(self.idx)?;

                    if let Some(next_c) = iter.peek() {
                        if next_c.is_digit(10) {
                            state.number = true;
                            state.symbol.push(c);
                            continue;
                        }
                    }

                    let op = Self::operator(c, self.idx)?;
                    let item = LexerAny::LexerOperator(op);
                    state.list.push(item);
                }

                '+' | '*' | '/' | '=' | '%' => {
                    state.delimit(self.idx)?;

                    let op = Self::operator(c, self.idx)?;
                    let item = LexerAny::LexerOperator(op);
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
                        let msg = format!("{}: unrecognized character \'{}\'", self.idx, c);
                        return Err(RuntimeError::Lexer(msg));
                    }
                }
            }

            self.idx += 1;
        }

        state.delimit(self.idx)?;

        Ok(state.list)
    }

    fn operator(c: char, idx: usize) -> RuntimeResult<LexerOperator> {
        match c {
            '+' => Ok(LexerOperator::Add),
            '=' => Ok(LexerOperator::Assign),
            '-' => Ok(LexerOperator::Subtract),
            '*' => Ok(LexerOperator::Multiply),
            '/' => Ok(LexerOperator::Divide),
            '%' => Ok(LexerOperator::Modulo),
            _ => {
                let msg = format!("{}: unrecognized operator \'{}\'", idx, c);
                Err(RuntimeError::Lexer(msg))
            }
        }
    }

    fn munch(iter: &mut Peekable<Chars>) -> usize {
        let mut idx = 0;

        for c in iter {
            if c == '\n' || c == '\r' {
                break;
            }
            idx += 1;
        }

        idx
    }

    fn get_bin_expr(&mut self, iter: &mut Peekable<Chars>) -> RuntimeResult<LexerBinExpr> {
        let expr_list = self.lex(iter)?;
        let mut expr_iter = expr_list.iter();

        let a = Self::get_expression(expr_iter.next(), self.idx)?;
        let op = Self::get_op_item(expr_iter.next(), self.idx)?;
        let b = Self::get_expression(expr_iter.next(), self.idx)?;

        Ok(LexerBinExpr::new(a, op, b))
    }

    fn get_expression(item: Option<&LexerAny>, idx: usize) -> RuntimeResult<LexerAny> {
        match item {
            Some(LexerAny::LexerBinExpr(bin_expr)) => Ok(LexerAny::LexerBinExpr(bin_expr.clone())),
            Some(LexerAny::LexerList(list)) => Ok(LexerAny::LexerList(list.clone())),
            Some(LexerAny::LexerNumber(num)) => Ok(LexerAny::LexerNumber(*num)),
            Some(LexerAny::LexerWord(word)) => Ok(LexerAny::LexerWord(word.clone())),
            _ => {
                let msg = format!("{}: expected an expression", idx);
                Err(RuntimeError::Lexer(msg))
            }
        }
    }

    fn get_op_item(item: Option<&LexerAny>, idx: usize) -> RuntimeResult<LexerOperator> {
        if let Some(LexerAny::LexerOperator(op)) = item {
            Ok(*op)
        } else {
            let msg = format!("{}: expected an operator", idx);
            Err(RuntimeError::Lexer(msg))
        }
    }
}
