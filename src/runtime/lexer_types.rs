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
pub struct Word {
    name: String,
}

impl Word {
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
pub struct Number {
    val: f64,
}

impl Number {
    pub fn new(val: f64) -> Self {
        Self { val }
    }

    pub fn val(&self) -> f64 {
        self.val
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Assign,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    a: Box<ExprNumWord>,
    op: Operator,
    b: Box<ExprNumWord>,
}

impl Expression {
    pub fn new(a: ExprNumWord, op: Operator, b: ExprNumWord) -> Self {
        Self {
            a: Box::new(a),
            op,
            b: Box::new(b),
        }
    }

    pub fn a(&self) -> &ExprNumWord {
        &self.a
    }

    pub fn op(&self) -> Operator {
        self.op
    }

    pub fn b(&self) -> &ExprNumWord {
        &self.b
    }
}

pub type List = Vec<AnyItem>;

pub type Block = List;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprNumWord {
    Expression(Expression),
    Number(Number),
    Word(Word),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListNumWord {
    List(List),
    Number(Number),
    Word(Word),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnyItem {
    Block(Block),
    Expression(Expression),
    ExprNumWord(ExprNumWord),
    List(List),
    ListNumWord(ListNumWord),
    Number(Number),
    Operator(Operator),
    Word(Word),
}
