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
    LexerUnbalancedList,
    LexerUnexpectedQuote,
    LexerUnexpectedValueOf,
    LexerUnrecognizedCharacter,

    ParserDuplicateProcedure,
    ParserExpectedArgument,
    ParserExpectedEnd,
    ParserExpectedList,
    ParserExpectedProcedure,
    ParserExpectedQuoted,
    ParserUnexpectedEnd,
    ParserUnrecognizedProcedure,
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match *self {
            Self::LexerMaxStack => "[Lexer] Maximum stack size exceeded",
            Self::LexerUnbalancedList => "[Lexer] Unbalanced list",
            Self::LexerUnexpectedQuote => "[Lexer] Unexpected quote",
            Self::LexerUnexpectedValueOf => "[Lexer] Unexpected value of",
            Self::LexerUnrecognizedCharacter => "[Lexer] Unrecognized character",

            Self::ParserDuplicateProcedure => "",
            Self::ParserExpectedArgument => "[Parser] Expected argument",
            Self::ParserExpectedEnd => "",
            Self::ParserExpectedList => "[Parser] Expected list",
            Self::ParserExpectedProcedure => "[Parser] Expected procedure",
            Self::ParserExpectedQuoted => "[Parser] Expected quoted word",
            Self::ParserUnexpectedEnd => "[Parser] Unexpected end of input",
            Self::ParserUnrecognizedProcedure => "[Parser] Unrecognized procedure",
        };

        write!(f, "{}", s)
    }
}
