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

    fn expect_assign(&mut self) -> RuntimeResult {
        if let LexerAny::LexerOperator(op) = self.next() {
            if op == LexerOperator::Assign {
                return Ok(());
            }
        }

        let msg = "expected an assignment".to_string();
        Err(RuntimeError::Parser(msg))
    }

    fn next(&mut self) -> LexerAny {
        let temp = self.idx;
        self.idx += 1;
        self.list[temp].clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            let word = self.get_word(iter)?;
            let node = self.parse_word(iter, &word)?;
            list.push(node);
        }

        Ok(list)
    }

    fn parse_word(&mut self, iter: &mut ListIter, word: &str) -> RuntimeResult<ParserNode> {
        let res = match word.to_lowercase().as_str() {
            "bk" | "backward" => self.parse_backward(iter)?,
            "clean" => self.parse_clean(),
            "cs" | "clearscreen" => self.parse_clear_screen(),
            "fd" | "forward" => self.parse_forward(iter)?,
            "fn" => self.parse_fn(iter)?,
            "ht" | "hideturtle" => ParserNode::ShowTurtle(false),
            "home" => self.parse_home(),
            "let" => self.parse_let(iter)?,
            "lt" | "left" => self.parse_left(iter)?,
            "pd" | "pendown" => self.parse_pen_down(),
            "pu" | "penup" => self.parse_pen_up(),
            "random" => self.parse_random(iter)?,
            "repcount" => ParserNode::Repcount,
            "repeat" => self.parse_repeat(iter)?,
            "rt" | "right" => self.parse_right(iter)?,
            "seth" | "setheading" => self.parse_set_heading(iter)?,
            "setpc" | "setpencolor" => self.parse_set_pen_color(iter)?,
            "setpos" => self.parse_set_pos(iter)?,
            "setsc" | "setscreencolor" => self.parse_set_screen_color(iter)?,
            "setxy" => self.parse_setxy(iter)?,
            "setx" => self.parse_setx(iter)?,
            "sety" => self.parse_sety(iter)?,
            "st" | "showturtle" => ParserNode::ShowTurtle(true),
            _ => self.parse_other(iter, word)?,
        };

        Ok(res)
    }

    fn parse_other(&mut self, iter: &mut ListIter, word: &str) -> RuntimeResult<ParserNode> {
        match self.smap.get(word) {
            Some(SymbolTag::Func) => self.parse_call(iter, word),
            Some(SymbolTag::Var) => Ok(ParserNode::Word(word.to_string())),
            _ => {
                let msg = format!("unrecognized symbol \"{}\"", word);
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn parse_backward(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let distance = self.get_expr(iter)?;
        let distance_node = self.parse_expr(iter, &distance)?;
        let move_node = MoveNode::new(distance_node, Direction::Backward);
        Ok(ParserNode::Move(move_node))
    }

    fn parse_bin_expr(
        &mut self,
        iter: &mut ListIter,
        bin_expr: &LexerBinExpr,
    ) -> RuntimeResult<ParserNode> {
        let a = bin_expr.a();
        let b = bin_expr.b();
        let anode = self.parse_expr(iter, a)?;
        let bnode = self.parse_expr(iter, b)?;
        let node = BinExprNode::new(anode, bin_expr.op(), bnode);
        Ok(ParserNode::BinExpr(node))
    }

    fn parse_call(&mut self, iter: &mut ListIter, name: &str) -> RuntimeResult<ParserNode> {
        let func_def = self.fmap.get(name).unwrap();
        let num_args = func_def.num_args();
        iter.expect(num_args)?;
        let args = self.get_args(iter, num_args)?;
        let call = CallNode::new(name, args);
        Ok(ParserNode::Call(call))
    }

    fn parse_clean(&mut self) -> ParserNode {
        ParserNode::Clean
    }

    fn parse_clear_screen(&mut self) -> ParserNode {
        ParserNode::ClearScreen
    }

    fn parse_expr(&mut self, iter: &mut ListIter, expr: &LexerAny) -> RuntimeResult<ParserNode> {
        match expr {
            LexerAny::LexerBinExpr(bin_expr) => self.parse_bin_expr(iter, &bin_expr),
            LexerAny::LexerNumber(num) => Ok(ParserNode::Number(*num)),
            LexerAny::LexerList(list) => self.parse_list(&list),
            LexerAny::LexerWord(word) => self.parse_word(iter, &word),
            _ => {
                let msg = "failed to parse expression".to_string();
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn parse_fn(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        let name = self.get_word(iter)?;
        self.check_symbol(&name, SymbolTag::Func)?;
        let block = self.get_block(iter)?;
        let mut block_iter = ListIter::new(&block);
        let list = self.parse(&mut block_iter)?;
        let func = ParserFuncDef::new(false, 0, list);
        self.fmap.insert(name, func);
        Ok(ParserNode::Placeholder)
    }

    fn parse_forward(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let distance = self.get_expr(iter)?;
        let distance_node = self.parse_expr(iter, &distance)?;
        let move_node = MoveNode::new(distance_node, Direction::Forward);
        Ok(ParserNode::Move(move_node))
    }

    fn parse_home(&mut self) -> ParserNode {
        ParserNode::Home
    }

    fn parse_let(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(3)?;
        let var = self.get_word(iter)?;
        self.check_symbol(&var, SymbolTag::Var)?;
        iter.expect_assign()?;
        let rhs = iter.next();
        let rhs_node = self.parse_expr(iter, &rhs)?;
        let l_node = LetNode::new(var, rhs_node);
        Ok(ParserNode::Let(l_node))
    }

    fn parse_left(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = self.get_expr(iter)?;
        let angle_node = self.parse_expr(iter, &angle)?;
        let rotate_node = RotateNode::new(angle_node, Direction::Left);
        Ok(ParserNode::Rotate(rotate_node))
    }

    fn parse_list(&mut self, list: &LexerList) -> RuntimeResult<ParserNode> {
        let mut list_iter = ListIter::new(&list);

        let mut node_list = ParserNodeList::new();
        while !list_iter.is_empty() {
            let expr = self.get_expr(&mut list_iter)?;
            let node = self.parse_expr(&mut list_iter, &expr)?;
            node_list.push(node);
        }
        Ok(ParserNode::List(node_list))
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
        let max = iter.next();
        let max_node = self.parse_expr(iter, &max)?;
        let random_node = RandomNode::new(max_node);
        Ok(ParserNode::Random(random_node))
    }

    fn parse_repeat(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        let count = self.get_expr(iter)?;
        let count_node = self.parse_expr(iter, &count)?;
        let block = self.get_block(iter)?;
        let mut block_iter = ListIter::new(&block);
        let node_list = self.parse(&mut block_iter)?;
        let repeat_node = RepeatNode::new(count_node, node_list);
        Ok(ParserNode::Repeat(repeat_node))
    }

    fn parse_right(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = self.get_expr(iter)?;
        let angle_node = self.parse_expr(iter, &angle)?;
        let rotate_node = RotateNode::new(angle_node, Direction::Right);
        Ok(ParserNode::Rotate(rotate_node))
    }

    fn parse_set_heading(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let angle = self.get_expr(iter)?;
        let angle_node = self.parse_expr(iter, &angle)?;
        let node = SetHeadingNode::new(angle_node);
        Ok(ParserNode::SetHeading(node))
    }

    fn parse_set_pen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let color = self.get_expr(iter)?;
        let color_node = self.parse_expr(iter, &color)?;
        let pen_color_node = SetPenColorNode::new(color_node);
        Ok(ParserNode::SetPenColor(pen_color_node))
    }

    fn parse_set_pos(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let pos = self.get_list(iter)?;
        let mut pos_iter = ListIter::new(&pos);
        self.parse_setxy(&mut pos_iter)
    }

    fn parse_set_screen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let color = self.get_expr(iter)?;
        let color_node = self.parse_expr(iter, &color)?;
        let pen_color_node = SetScreenColorNode::new(color_node);
        Ok(ParserNode::SetScreenColor(pen_color_node))
    }

    fn parse_setxy(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(2)?;
        let x = self.get_expr(iter)?;
        let x_node = self.parse_expr(iter, &x)?;
        let y = self.get_expr(iter)?;
        let y_node = self.parse_expr(iter, &y)?;
        let pos_node = SetPositionNode::new(Some(Box::new(x_node)), Some(Box::new(y_node)));
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn parse_setx(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let x = self.get_expr(iter)?;
        let x_node = self.parse_expr(iter, &x)?;
        let pos_node = SetPositionNode::new(Some(Box::new(x_node)), None);
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn parse_sety(&mut self, iter: &mut ListIter) -> RuntimeResult<ParserNode> {
        iter.expect(1)?;
        let y = self.get_expr(iter)?;
        let y_node = self.parse_expr(iter, &y)?;
        let pos_node = SetPositionNode::new(None, Some(Box::new(y_node)));
        Ok(ParserNode::SetPosition(pos_node))
    }

    fn get_args(&mut self, iter: &mut ListIter, num_args: usize) -> RuntimeResult<LexerList> {
        let mut args = LexerList::with_capacity(num_args as usize);
        for _ in 0..num_args {
            let arg = self.get_expr(iter)?;
            args.push(arg);
        }

        Ok(args)
    }

    fn get_block(&mut self, iter: &mut ListIter) -> RuntimeResult<LexerBlock> {
        if let LexerAny::LexerBlock(block) = iter.next() {
            Ok(block)
        } else {
            let msg = "expected a block".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_expr(&mut self, iter: &mut ListIter) -> RuntimeResult<LexerAny> {
        match iter.next() {
            LexerAny::LexerBinExpr(bin_expr) => Ok(LexerAny::LexerBinExpr(bin_expr)),
            LexerAny::LexerList(list) => Ok(LexerAny::LexerList(list)),
            LexerAny::LexerNumber(num) => Ok(LexerAny::LexerNumber(num)),
            LexerAny::LexerWord(word) => Ok(LexerAny::LexerWord(word)),
            _ => {
                let msg = "expected an expression".to_string();
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn get_list(&mut self, iter: &mut ListIter) -> RuntimeResult<LexerList> {
        if let LexerAny::LexerList(list) = iter.next() {
            Ok(list)
        } else {
            let msg = "expected a list".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_word(&mut self, iter: &mut ListIter) -> RuntimeResult<String> {
        if let LexerAny::LexerWord(word) = iter.next() {
            Ok(word)
        } else {
            let msg = "expected a word".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn check_symbol(&mut self, name: &str, tag: SymbolTag) -> RuntimeResult {
        if let Some(existing_tag) = self.smap.get(name) {
            if *existing_tag == tag {
                Ok(())
            } else {
                let msg = format!(
                    "symbol \"{}\" already exists with tag {:?}",
                    name, existing_tag
                );
                Err(RuntimeError::Parser(msg))
            }
        } else {
            self.smap.insert(name.to_string(), tag);
            Ok(())
        }
    }
}
