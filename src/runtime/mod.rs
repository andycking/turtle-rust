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
use error::*;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub mod error;
mod interpreter;
mod lexer;
mod lexer_types;
mod parser;
mod parser_types;

pub fn entry(input: String, render_tx: Arc<RenderTx>) -> RuntimeResult {
    println!("Runtime starting...");
    let lexer_out = Lexer::new().go(&input)?;
    let parser_out = Parser::new().go(&lexer_out)?;
    let intrp_out = Interpreter::new(render_tx).go(&parser_out)?;
    println!("Runtime finished.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use futures::channel::mpsc;

    use super::*;
    use crate::model::render::RenderCommand;

    #[test]
    fn it_goes() {
        let input = "fd".to_string();
        let (render_tx, render_rx) = mpsc::unbounded::<RenderCommand>();
        let res = entry(input, Arc::new(render_tx));
        println!("Result: {:?}", res);
    }
}
