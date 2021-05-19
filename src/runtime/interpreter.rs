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

use std::collections::VecDeque;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Copy, Debug)]
enum ObjectType {
    Word,
    List,
}

trait Object {
    fn object_type(&self) -> ObjectType;
    fn symbol(&self) -> &str;
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

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("[");

        for i in &self.items {
            s += i.symbol();
            s += ",";
        }

        s += "]";

        write!(f, "{}", s)
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

    pub fn push(&mut self, list: List) {
        self.items.push_front(list);
    }

    pub fn pop(&mut self) -> Option<List> {
        self.items.pop_front()
    }
}

pub fn go(input: &str) {
    let mut stack = Stack::new();
    let mut list = List::new();
    let mut depth = 1;

    for l in input.lines() {
        let mut word: String = String::new();

        let trimmed = l.trim();
        for c in trimmed.chars() {
            match c {
                ';' => {
                    if !word.is_empty() {
                        let obj = Word::new(&word);
                        list.push(Box::new(obj));
                    }

                    break;
                }

                '[' => {
                    if !word.is_empty() {
                        let obj = Word::new(&word);
                        list.push(Box::new(obj));
                        word = String::new();
                    }

                    stack.push(list);

                    list = List::new();
                    depth += 1;
                }

                ']' => {
                    if !word.is_empty() {
                        let obj = Word::new(&word);
                        list.push(Box::new(obj));
                        word = String::new();
                    }

                    if let Some(mut parent_list) = stack.pop() {
                        parent_list.push(Box::new(list));
                        list = parent_list;
                        depth -= 1;
                    } else {
                    }
                }

                '(' => {}

                ')' => {}

                '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                    word.push(c);
                }

                _ => {
                    if c.is_whitespace() {
                        if !word.is_empty() {
                            let obj = Word::new(&word);
                            list.push(Box::new(obj));
                            word = String::new();
                        }
                    } else if c.is_alphanumeric() {
                        word.push(c);
                    }
                }
            }
        }

        if !word.is_empty() {
            let obj = Word::new(&word);
            list.push(Box::new(obj));
        }
    }

    println!("{}", list);
}
