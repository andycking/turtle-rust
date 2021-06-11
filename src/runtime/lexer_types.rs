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

#[derive(Clone, Debug, PartialEq)]
pub struct LexerWord {
    name: String,
}

impl LexerWord {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LexerNumber {
    val: f64,
}

impl LexerNumber {
    pub fn new(val: f64) -> Self {
        Self { val }
    }

    pub fn val(&self) -> f64 {
        self.val
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LexerOperator {
    Add,
    Assign,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexerBinExpr {
    a: Box<LexerExpr>,
    op: LexerOperator,
    b: Box<LexerExpr>,
}

impl LexerBinExpr {
    pub fn new(a: LexerExpr, op: LexerOperator, b: LexerExpr) -> Self {
        Self {
            a: Box::new(a),
            op,
            b: Box::new(b),
        }
    }

    pub fn a(&self) -> &LexerExpr {
        &self.a
    }

    pub fn op(&self) -> LexerOperator {
        self.op
    }

    pub fn b(&self) -> &LexerExpr {
        &self.b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexerCall {
    name: String,
    args: LexerExprList,
}

impl LexerCall {
    pub fn new(name: &str, args: LexerExprList) -> Self {
        Self {
            name: String::from(name),
            args,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub type LexerList = Vec<LexerAny>;

pub type LexerBlock = LexerList;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerExpr {
    LexerBinExpr(LexerBinExpr),
    LexerCall(LexerCall),
    LexerList(LexerList),
    LexerNumber(LexerNumber),
    LexerWord(LexerWord),
}

pub type LexerExprList = Vec<LexerExpr>;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerAny {
    LexerBlock(LexerBlock),
    LexerBinExpr(LexerBinExpr),
    LexerCall(LexerCall),
    LexerExpr(LexerExpr),
    LexerList(LexerList),
    LexerNumber(LexerNumber),
    LexerOperator(LexerOperator),
    LexerWord(LexerWord),
}
