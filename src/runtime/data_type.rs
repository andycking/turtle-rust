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

use std::any::Any;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Copy, Debug)]
pub enum Tag {
    Word,
    List,
}

pub trait DataType {
    fn tag(&self) -> Tag;
    fn symbol(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, PartialEq)]
pub enum WordAttr {
    Basic,
    Literal,
    Variable,
}

impl Debug for WordAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match *self {
            Self::Basic => "b",
            Self::Literal => "l",
            Self::Variable => "v",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    symbol: String,
    attr: WordAttr,
}

impl DataType for Word {
    fn tag(&self) -> Tag {
        Tag::Word
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
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
}

pub type ListItems = Vec<Box<dyn DataType>>;

pub struct List {
    items: ListItems,
}

impl DataType for List {
    fn tag(&self) -> Tag {
        Tag::List
    }

    fn symbol(&self) -> &str {
        "LIST"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.items)
    }
}

impl Deref for List {
    type Target = ListItems;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl From<ListItems> for List {
    fn from(items: ListItems) -> Self {
        Self { items }
    }
}

impl List {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn consume(&mut self) -> ListItems {
        std::mem::replace(&mut self.items, Vec::new())
    }
}

impl Debug for dyn DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.tag() {
            Tag::Word => {
                let word = self.as_any().downcast_ref::<Word>().unwrap();
                write!(f, "{} ({:?})", word.symbol(), word.attr())
            }

            Tag::List => {
                let list = self.as_any().downcast_ref::<List>().unwrap();
                write!(f, "{:?}", list)
            }
        }
    }
}
