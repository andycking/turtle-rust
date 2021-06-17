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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LexerOperator {
    Add,
    Assign,
    Divide,
    Modulo,
    Multiply,
    Subtract,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexerBinExpr {
    a: Box<LexerAny>,
    op: LexerOperator,
    b: Box<LexerAny>,
}

impl LexerBinExpr {
    pub fn new(a: LexerAny, op: LexerOperator, b: LexerAny) -> Self {
        Self {
            a: Box::new(a),
            op,
            b: Box::new(b),
        }
    }

    pub fn a(&self) -> &LexerAny {
        &self.a
    }

    pub fn op(&self) -> LexerOperator {
        self.op
    }

    pub fn b(&self) -> &LexerAny {
        &self.b
    }
}

pub type LexerList = Vec<LexerAny>;

pub type LexerBlock = LexerList;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerAny {
    LexerBlock(LexerBlock),
    LexerBinExpr(LexerBinExpr),
    LexerList(LexerList),
    LexerNumber(f64),
    LexerOperator(LexerOperator),
    LexerWord(String),
}
