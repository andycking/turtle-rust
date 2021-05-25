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

use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WordAttr {
    Bare,
    Quoted,
    ValueOf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    symbol: String,
    attr: WordAttr,
}

impl Word {
    pub fn new(symbol: &str, attr: WordAttr) -> Self {
        Self {
            symbol: String::from(symbol),
            attr,
        }
    }

    pub fn attr(&self) -> WordAttr {
        self.attr
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Clone, Debug)]
pub struct List {
    items: Vec<DataType>,
}

impl List {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn consume(&mut self) -> Vec<DataType> {
        std::mem::replace(&mut self.items, Vec::new())
    }
}

impl Deref for List {
    type Target = Vec<DataType>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[derive(Clone, Debug)]
pub enum DataType {
    Word(Word),
    List(List),
}

impl From<Vec<DataType>> for List {
    fn from(items: Vec<DataType>) -> Self {
        Self { items }
    }
}
