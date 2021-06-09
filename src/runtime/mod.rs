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

use std::sync::Arc;

use crate::model::render::RenderTx;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub mod error;
mod interpreter;
mod lexer;
mod lexer_types;
mod parser;
mod parser_types;

pub async fn entry(input: String, render_tx: Arc<RenderTx>) {
    println!("Runtime starting...");
    match Lexer::new().go(&input) {
        Ok(lexer_out) => match Parser::new().go(&lexer_out) {
            Ok(parser_out) => match Interpreter::new(render_tx).go(&parser_out) {
                Ok(()) => {
                    println!("Runtime finished.");
                }
                Err(err) => {
                    eprintln!("Runtime: {}", err);
                }
            },
            Err(err) => {
                eprintln!("Runtime: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Runtime: {}", err);
        }
    }
}
