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

#[derive(Clone, Debug, PartialEq)]
struct Word {
    symbol: String,
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
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: String::from(symbol),
        }
    }
}

impl Deref for Word {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

struct List {
    items: Vec<Box<dyn Object>>,
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
    type Target = Vec<Box<dyn Object>>;

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
                write!(f, "{}", word.symbol())
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

impl List {
    pub fn new() -> Self {
        Self { items: Vec::new() }
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
    UnrecognizedCharacter,
    UnbalancedList,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::NoInput => "No input",
            Self::UnrecognizedCharacter => "Unrecognized character",
            Self::UnbalancedList => "Unbalanced list",
        };

        write!(f, "{}", s)
    }
}

fn delimit(word: &str, list: &mut List) -> String {
    if !word.is_empty() {
        let obj = Word::new(&word);
        list.push(Box::new(obj));
    }

    String::new()
}

fn open_list(list: List, stack: &mut Stack) -> List {
    stack.push_front(list);
    List::new()
}

fn close_list(list: List, stack: &mut Stack) -> Result<List, InterpreterError> {
    if let Some(mut parent_list) = stack.pop_front() {
        parent_list.push(Box::new(list));
        Ok(parent_list)
    } else {
        Err(InterpreterError::UnbalancedList)
    }
}

pub fn go(input: &str) -> Result<(), InterpreterError> {
    let mut stack = Stack::new();
    let mut list = List::new();

    for l in input.lines() {
        let mut word: String = String::new();

        let trimmed = l.trim();
        for c in trimmed.chars() {
            match c {
                ';' => {
                    word = delimit(&word, &mut list);
                    break;
                }

                '[' => {
                    word = delimit(&word, &mut list);
                    list = open_list(list, &mut stack);
                }

                ']' => {
                    word = delimit(&word, &mut list);
                    list = close_list(list, &mut stack)?;
                }

                '(' => {}

                ')' => {}

                '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                    word.push(c);
                }

                _ => {
                    if c.is_whitespace() {
                        word = delimit(&word, &mut list);
                    } else if c.is_alphanumeric() {
                        word.push(c);
                    } else {
                        return Err(InterpreterError::UnrecognizedCharacter);
                    }
                }
            }
        }

        delimit(&word, &mut list);
    }

    if !stack.is_empty() {
        return Err(InterpreterError::UnbalancedList);
    }

    println!("{:?}", list);

    Ok(())
}
