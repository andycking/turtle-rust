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

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum InterpreterError {
    LexerMaxStack,
    LexerNoInput,
    LexerUnbalancedList,
    LexerUnexpectedLiteral,
    LexerUnexpectedVariable,
    LexerUnrecognizedCharacter,

    ParserExpectedCommand,
    ParserExpectedDistance,
    ParserNoInput,
    ParserUnexpectedEnd,
    ParserUnrecognizedCommand,
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match *self {
            Self::LexerMaxStack => "[Lexer] Maximum stack size exceeded",
            Self::LexerNoInput => "[Lexer] No input",
            Self::LexerUnbalancedList => "[Lexer] Unbalanced list",
            Self::LexerUnexpectedLiteral => "[Lexer] Unexpected literal",
            Self::LexerUnexpectedVariable => "[Lexer] Unexpected variable",
            Self::LexerUnrecognizedCharacter => "[Lexer] Unrecognized character",

            Self::ParserExpectedCommand => "[Parser] Expected command",
            Self::ParserExpectedDistance => "[Parser] Expected distance",
            Self::ParserNoInput => "[Parser] No input",
            Self::ParserUnexpectedEnd => "[Parser] Unexpected end of input",
            Self::ParserUnrecognizedCommand => "[Parser] Unrecognized command",
        };

        write!(f, "{}", s)
    }
}
