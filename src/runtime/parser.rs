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

use std::collections::HashMap;

use super::error::*;
use super::lexer_types::*;
use super::parser_types::*;

#[derive(Clone, Debug)]
struct ListIter<'a> {
    list: &'a [LexerAny],
    idx: usize,
    depth: usize,
}

impl<'a> ListIter<'a> {
    pub fn new(list: &'a [LexerAny]) -> Self {
        Self {
            list,
            idx: 0,
            depth: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.idx >= self.list.len()
    }

    fn expect(&self, n: usize) -> RuntimeResult {
        if self.idx + n > self.list.len() {
            let msg = format!("{} items expected", n);
            Err(RuntimeError::Parser(msg))
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> LexerAny {
        let temp = self.idx;
        self.idx += 1;
        self.list[temp].clone()
    }

    fn get_args(&mut self, num_args: usize) -> RuntimeResult<LexerExprList> {
        let mut args = LexerExprList::with_capacity(num_args as usize);
        for _ in 0..num_args {
            let arg = self.get_expression()?;
            args.push(arg);
        }

        Ok(args)
    }

    fn get_assignment(&mut self) -> RuntimeResult<LexerOperator> {
        let op = self.get_operator()?;
        if op == LexerOperator::Assign {
            Ok(op)
        } else {
            let msg = "expected an assignment".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_block(&mut self) -> RuntimeResult<LexerBlock> {
        if let LexerAny::LexerBlock(block) = self.next() {
            Ok(block)
        } else {
            let msg = "expected a block".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_expression(&mut self) -> RuntimeResult<LexerExpr> {
        match self.next() {
            LexerAny::LexerBinExpr(bin_expr) => Ok(LexerExpr::LexerBinExpr(bin_expr)),
            LexerAny::LexerList(list) => Ok(LexerExpr::LexerList(list)),
            LexerAny::LexerNumber(num) => Ok(LexerExpr::LexerNumber(num)),
            LexerAny::LexerWord(word) => Ok(LexerExpr::LexerWord(word)),
            _ => {
                let msg = "expected an expression".to_string();
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn get_list(&mut self) -> RuntimeResult<LexerList> {
        if let LexerAny::LexerList(list) = self.next() {
            Ok(list)
        } else {
            let msg = "expected a list".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_operator(&mut self) -> RuntimeResult<LexerOperator> {
        if let LexerAny::LexerOperator(op) = self.next() {
            Ok(op)
        } else {
            let msg = "expected an operator".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_word(&mut self) -> RuntimeResult<String> {
        if let LexerAny::LexerWord(word) = self.next() {
            Ok(word)
        } else {
            let msg = "expected a word".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }
}

#[derive(Clone, Debug)]
enum SymbolTag {
    Func,
    Var,
}

#[derive(Clone, Debug)]
pub struct Parser {
    smap: HashMap<String, SymbolTag>,
    fmap: ParserFuncMap,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            smap: HashMap::new(),
            fmap: ParserFuncMap::new(),
        }
    }

    pub fn go(&mut self, input: &[LexerAny]) -> RuntimeResult<ParserOutput> {
        let mut iter = ListIter::new(input);
        let list = self.parse(&mut iter)?;
        Ok(ParserOutput::new(list, self.fmap.to_owned()))
    }

    fn parse(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNodeList> {
        let mut list = ParserNodeList::new();

        while !iter.is_empty() {
            let word = iter.get_word()?;
            let res = self.parse_word(iter, &word)?;
            if let Some(node) = res {
                list.push(node);
            }
        }

        Ok(list)
    }

    fn parse_word(&mut self, iter: &mut ListIter, word: &str) -> RuntimeResult<Option<ParserNode>> {
        let res = match word.to_lowercase().as_str() {
            "bk" | "backward" => Some(self.parse_backward(iter)?),
            "clean" => Some(self.parse_clean()),
            "cs" | "clearscreen" => Some(self.parse_clear_screen()),
            "fd" | "forward" => Some(self.parse_forward(iter)?),
            "fn" => {
                self.parse_fn(iter)?;
                None
            }
            "home" => Some(self.parse_home()),
            "let" => Some(self.parse_let(iter)?),
            "lt" | "left" => Some(self.parse_left(iter)?),
            "pd" | "pendown" => Some(self.parse_pen_down()),
            "pu" | "penup" => Some(self.parse_pen_up()),
            "random" => Some(self.parse_random(iter)?),
            "repeat" => Some(self.parse_repeat(iter)?),
            "rt" | "right" => Some(self.parse_right(iter)?),
            "seth" | "setheading" => Some(self.parse_set_heading(iter)?),
            "setpc" | "setpencolor" => Some(self.parse_set_pen_color(iter)?),
            "setpos" => Some(self.parse_set_pos(iter)?),
            "setsc" | "setscreencolor" => Some(self.parse_set_screen_color(iter)?),
            "setxy" => Some(self.parse_setxy(iter)?),
            "setx" => Some(self.parse_setx(iter)?),
            "sety" => Some(self.parse_sety(iter)?),
            _ => Some(self.parse_other(iter, word)?),
        };

        Ok(res)
    }

    fn parse_other(&mut self, iter: &mut ListIter, word: &str) -> RuntimeResult<ParserNode> {
        match self.smap.get(word) {
            Some(SymbolTag::Func) => self.parse_call(iter, word),
            Some(SymbolTag::Var) => self.parse_assign(iter, word),
            _ => {
                let msg = format!("unrecognized symbol {}", word);
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn parse_assign(&mut self, iter: &mut ListIter, name: &str) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        iter.get_assignment()?;
        let rhs = iter.get_expression()?;
        let node = AssignNode::new(name.to_string(), rhs);
        Ok(ParserNode::Assign(node))
    }

    fn parse_backward(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let distance = iter.get_expression()?;
        let move_node = MoveNode::new(distance, Direction::Backward);
        Ok(ParserNode::Move(move_node))
    }

    fn parse_call(&mut self, iter: &mut ListIter, name: &str) -> RuntimeResult<ParserNode> {
        let func_def = self.fmap.get(name).unwrap();
        let num_args = func_def.num_args();
        iter.expect(num_args)?;
        let args = iter.get_args(num_args)?;
        let call = LexerCall::new(name, args);
        Ok(ParserNode::Call(call))
    }

    fn parse_clean(&mut self) -> ParserNode {
        ParserNode::Clean
    }

    fn parse_clear_screen(&mut self) -> ParserNode {
        ParserNode::ClearScreen
    }

    fn parse_fn(&mut self, iter: &mut ListIter) -> RuntimeResult {
        iter.expect(2)?;
        let name = iter.get_word()?;
        self.check_symbol(&name, SymbolTag::Func)?;
        let block = iter.get_block()?;
        let mut block_iter = ListIter::new(&block);
        let list = self.parse(&mut block_iter)?;
        let func = ParserFuncDef::new(false, 0, list);
        self.fmap.insert(name, func);
        Ok(())
    }

    fn parse_forward(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let distance = iter.get_expression()?;
        let move_node = MoveNode::new(distance, Direction::Forward);
        Ok(ParserNode::Move(move_node))
    }

    fn parse_home(&mut self) -> ParserNode {
        ParserNode::Home
    }

    fn parse_let(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(3)?;
        let var = iter.get_word()?;
        self.check_symbol(&var, SymbolTag::Var)?;
        iter.get_assignment()?;
        let rhs = iter.get_expression()?;
        let l_node = LetNode::new(var, rhs);
        Ok(ParserNode::Let(l_node))
    }

    fn parse_left(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let rotate_node = RotateNode::new(angle, Direction::Left);
        Ok(ParserNode::Rotate(rotate_node))
    }

    fn parse_pen_down(&mut self) -> ParserNode {
        let pen_node = PenNode::Down;
        ParserNode::Pen(pen_node)
    }

    fn parse_pen_up(&mut self) -> ParserNode {
        let pen_node = PenNode::Up;
        ParserNode::Pen(pen_node)
    }

    fn parse_random(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let max = iter.get_expression()?;
        Ok(ParserNode::Random(max))
    }

    fn parse_repeat(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        let count = iter.get_expression()?;
        let block = iter.get_block()?;
        let mut block_iter = ListIter::new(&block);
        let node_list = self.parse(&mut block_iter)?;
        let repeat_node = RepeatNode::new(count, node_list);
        Ok(ParserNode::Repeat(repeat_node))
    }

    fn parse_right(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let rotate_node = RotateNode::new(angle, Direction::Right);
        Ok(ParserNode::Rotate(rotate_node))
    }

    fn parse_set_heading(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let node = SetHeadingNode::new(angle);
        Ok(ParserNode::SetHeading(node))
    }

    fn parse_set_pen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let color = iter.get_expression()?;
        let pen_color_node = SetPenColorNode::new(color);
        Ok(ParserNode::SetPenColor(pen_color_node))
    }

    fn parse_set_pos(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let pos = iter.get_list()?;
        let mut pos_iter = ListIter::new(&pos);
        self.parse_setxy(&mut pos_iter)
    }

    fn parse_set_screen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let color = iter.get_expression()?;
        let pen_color_node = SetScreenColorNode::new(color);
        Ok(ParserNode::SetScreenColor(pen_color_node))
    }

    fn parse_setxy(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        let x = iter.get_expression()?;
        let y = iter.get_expression()?;
        let pos_node = SetPositionNode::new(Some(x), Some(y));
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn parse_setx(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let x = iter.get_expression()?;
        let pos_node = SetPositionNode::new(Some(x), None);
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn parse_sety(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let y = iter.get_expression()?;
        let pos_node = SetPositionNode::new(None, Some(y));
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn check_symbol(&mut self, name: &str, tag: SymbolTag) -> RuntimeResult {
        if !self.smap.contains_key(name) {
            self.smap.insert(name.to_string(), tag);
            Ok(())
        } else {
            let msg = format!("duplicate symbol {}", name);
            Err(RuntimeError::Parser(msg))
        }
    }
}
