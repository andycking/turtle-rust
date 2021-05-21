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
use std::collections::VecDeque;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::result::Result;

#[derive(Clone, Copy, Debug)]
enum ObjectType {
    Word,
    List,
}

trait Object {
    fn object_type(&self) -> ObjectType;
    fn symbol(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, PartialEq)]
enum WordAttr {
    Basic,
    Literal,
    Variable,
}

impl fmt::Debug for WordAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::Basic => "b",
            Self::Literal => "l",
            Self::Variable => "v",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Word {
    symbol: String,
    attr: WordAttr,
}

impl Object for Word {
    fn object_type(&self) -> ObjectType {
        ObjectType::Word
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

impl Deref for Word {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

type ListItems = Vec<Box<dyn Object>>;

struct List {
    items: ListItems,
}

impl Object for List {
    fn object_type(&self) -> ObjectType {
        ObjectType::List
    }

    fn symbol(&self) -> &str {
        "LIST"
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl fmt::Debug for dyn Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.object_type() {
            ObjectType::Word => {
                let word = self.as_any().downcast_ref::<Word>().unwrap();
                write!(f, "{} ({:?})", word.symbol(), word.attr())
            }

            ObjectType::List => {
                let list = self.as_any().downcast_ref::<List>().unwrap();
                write!(f, "{:?}", list)
            }
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.items)
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

struct Stack {
    items: VecDeque<List>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
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

#[derive(Debug)]
pub enum InterpreterError {
    NoInput,
    UnbalancedList,
    UnexpectedLiteral,
    UnexpectedVariable,
    UnrecognizedCharacter,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::NoInput => "No input",
            Self::UnbalancedList => "Unbalanced list",
            Self::UnexpectedLiteral => "Unexpected literal",
            Self::UnexpectedVariable => "Unexpected variable",
            Self::UnrecognizedCharacter => "Unrecognized character",
        };

        write!(f, "{}", s)
    }
}

struct Info {
    symbol: String,
    attr: WordAttr,
    list: List,
    stack: Stack,
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {:?}", self.symbol, self.attr, self.list)
    }
}

impl Info {
    pub fn new() -> Self {
        Self {
            symbol: String::new(),
            attr: WordAttr::Basic,
            list: List::new(),
            stack: Stack::new(),
        }
    }

    pub fn reset(&mut self) {
        self.symbol = String::new();
        self.attr = WordAttr::Basic;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn append_char(&mut self, c: char) {
        self.symbol.push(c);
    }

    pub fn delimit(&mut self) {
        if !self.symbol.is_empty() {
            let obj = Word::new(&self.symbol, self.attr);
            self.list.push(Box::new(obj));
        }

        self.reset();
    }

    pub fn attr(&self) -> WordAttr {
        self.attr
    }

    pub fn set_attr(&mut self, attr: WordAttr) {
        self.attr = attr;
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn open_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        self.stack.push_front(List::from(items));
    }

    pub fn close_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        let child = List::from(items);

        let mut parent = self.stack.pop_front().unwrap();
        parent.push(Box::new(child));

        self.list = parent;
    }
}

pub fn go(input: &str) -> Result<(), InterpreterError> {
    if input.is_empty() {
        return Err(InterpreterError::NoInput);
    }

    let mut info = Info::new();

    for l in input.lines() {
        let trimmed = l.trim();
        for c in trimmed.chars() {
            match c {
                ';' => {
                    info.delimit();
                    break;
                }

                '[' => {
                    info.delimit();
                    info.open_list();
                }

                ']' => {
                    if info.depth() == 0 {
                        return Err(InterpreterError::UnbalancedList);
                    }

                    info.delimit();
                    info.close_list();
                }

                '(' => {}

                ')' => {}

                '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                    info.append_char(c);
                }

                '\u{0022}' => {
                    if !info.symbol().is_empty() {
                        return Err(InterpreterError::UnexpectedLiteral);
                    }

                    info.set_attr(WordAttr::Literal);
                }

                ':' => {
                    if !info.symbol().is_empty() {
                        return Err(InterpreterError::UnexpectedVariable);
                    }

                    info.set_attr(WordAttr::Variable);
                }

                _ => {
                    if c.is_whitespace() {
                        info.delimit();
                    } else if c.is_alphanumeric() {
                        info.append_char(c);
                    } else {
                        return Err(InterpreterError::UnrecognizedCharacter);
                    }
                }
            }
        }

        info.delimit();
    }

    if info.depth() > 0 {
        return Err(InterpreterError::UnbalancedList);
    }

    println!("{:?}", info);

    Ok(())
}
