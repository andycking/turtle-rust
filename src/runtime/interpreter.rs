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
    MaxStack,
    UnbalancedList,
    UnexpectedLiteral,
    UnexpectedVariable,
    UnrecognizedCharacter,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::NoInput => "No input",
            Self::MaxStack => "Maximum stack size exceeded",
            Self::UnbalancedList => "Unbalanced list",
            Self::UnexpectedLiteral => "Unexpected literal",
            Self::UnexpectedVariable => "Unexpected variable",
            Self::UnrecognizedCharacter => "Unrecognized character",
        };

        write!(f, "{}", s)
    }
}

struct TokenLexer {
    symbol: String,
    attr: WordAttr,
    list: List,
    stack: Stack,
}

impl fmt::Debug for TokenLexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {:?}", self.symbol, self.attr, self.list)
    }
}

impl TokenLexer {
    pub fn new() -> Self {
        Self {
            symbol: String::new(),
            attr: WordAttr::Basic,
            list: List::new(),
            stack: Stack::new(),
        }
    }

    fn reset(&mut self) {
        self.symbol = String::new();
        self.attr = WordAttr::Basic;
    }

    fn has_symbol(&self) -> bool {
        !self.symbol.is_empty()
    }

    fn append_char(&mut self, c: char) {
        self.symbol.push(c);
    }

    fn delimit(&mut self) {
        if !self.symbol.is_empty() {
            let obj = Word::new(&self.symbol, self.attr);
            self.list.push(Box::new(obj));
        }

        self.reset();
    }

    fn attr(&self) -> WordAttr {
        self.attr
    }

    fn set_attr(&mut self, attr: WordAttr) {
        self.attr = attr;
    }

    fn depth(&self) -> usize {
        self.stack.len()
    }

    fn open_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        self.stack.push_front(List::from(items));
    }

    fn close_list(&mut self) {
        self.reset();

        let items = self.list.consume();
        let child = List::from(items);

        let mut parent = self.stack.pop_front().unwrap();
        parent.push(Box::new(child));

        self.list = parent;
    }

    pub fn go(&mut self, input: &str) -> Result<List, InterpreterError> {
        if input.is_empty() {
            return Err(InterpreterError::NoInput);
        }

        for l in input.lines() {
            let trimmed = l.trim();
            for c in trimmed.chars() {
                match c {
                    ';' => {
                        self.delimit();
                        break;
                    }

                    '[' => {
                        if self.depth() == 32 {
                            return Err(InterpreterError::MaxStack);
                        }

                        self.delimit();
                        self.open_list();
                    }

                    ']' => {
                        if self.depth() == 0 {
                            return Err(InterpreterError::UnbalancedList);
                        }

                        self.delimit();
                        self.close_list();
                    }

                    '(' => {}

                    ')' => {}

                    '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                        self.append_char(c);
                    }

                    '\u{0022}' => {
                        if !self.has_symbol() {
                            return Err(InterpreterError::UnexpectedLiteral);
                        }

                        self.set_attr(WordAttr::Literal);
                    }

                    ':' => {
                        if !self.has_symbol() {
                            return Err(InterpreterError::UnexpectedVariable);
                        }

                        self.set_attr(WordAttr::Variable);
                    }

                    _ => {
                        if c.is_whitespace() {
                            self.delimit();
                        } else if c.is_alphanumeric() {
                            self.append_char(c);
                        } else {
                            return Err(InterpreterError::UnrecognizedCharacter);
                        }
                    }
                }
            }

            self.delimit();
        }

        if self.depth() > 0 {
            return Err(InterpreterError::UnbalancedList);
        }

        Ok(List::from(self.list.consume()))
    }
}

pub fn go(input: &str) -> Result<(), InterpreterError> {
    match TokenLexer::new().go(input) {
        Ok(list) => Ok(()),
        Err(e) => Err(e),
    }
}
