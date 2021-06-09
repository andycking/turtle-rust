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

use std::fmt;

use futures::channel::mpsc::TrySendError;

use crate::model::runtime::DrawCommand;

#[derive(Debug)]
pub enum RuntimeError {
    Lexer(String),
    Parser(String),
    Interpreter(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::Lexer(msg) => write!(f, "Lexer: {}", msg),
            RuntimeError::Parser(msg) => write!(f, "Parser: {}", msg),
            RuntimeError::Interpreter(msg) => write!(f, "Interpreter: {}", msg),
        }
    }
}

impl From<TrySendError<DrawCommand>> for RuntimeError {
    fn from(err: TrySendError<DrawCommand>) -> Self {
        Self::Interpreter(err.to_string())
    }
}

pub type RuntimeResult<T = ()> = Result<T, RuntimeError>;
