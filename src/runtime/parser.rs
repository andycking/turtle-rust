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
    list: &'a [AnyItem],
    idx: usize,
    depth: usize,
}

impl<'a> ListIter<'a> {
    pub fn new(list: &'a [AnyItem]) -> Self {
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

    fn next(&mut self) -> AnyItem {
        let temp = self.idx;
        self.idx += 1;
        self.list[temp].clone()
    }

    fn get_assignment(&mut self) -> RuntimeResult<Operator> {
        let op = self.get_operator()?;
        if op == Operator::Assign {
            Ok(op)
        } else {
            let msg = "expected an assignment".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_block(&mut self) -> RuntimeResult<Block> {
        if let AnyItem::Block(block) = self.next() {
            Ok(block)
        } else {
            let msg = "expected a block".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_expression(&mut self) -> RuntimeResult<Expression> {
        match self.next() {
            AnyItem::BinExpr(bin_expr) => Ok(Expression::BinExpr(bin_expr)),
            AnyItem::List(list) => Ok(Expression::List(list)),
            AnyItem::Number(num) => Ok(Expression::Number(num)),
            AnyItem::Word(word) => Ok(Expression::Word(word)),
            _ => {
                let msg = "expected an expression".to_string();
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn get_list(&mut self) -> RuntimeResult<List> {
        if let AnyItem::List(list) = self.next() {
            Ok(list)
        } else {
            let msg = "expected a list".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_list_num_word(&mut self) -> RuntimeResult<ListNumWord> {
        match self.next() {
            AnyItem::List(list) => Ok(ListNumWord::List(list)),
            AnyItem::Number(num) => Ok(ListNumWord::Number(num)),
            AnyItem::Word(word) => Ok(ListNumWord::Word(word)),
            _ => {
                let msg = "expected a list, number or word".to_string();
                Err(RuntimeError::Parser(msg))
            }
        }
    }

    fn get_operator(&mut self) -> RuntimeResult<Operator> {
        if let AnyItem::Operator(op) = self.next() {
            Ok(op)
        } else {
            let msg = "expected an operator".to_string();
            Err(RuntimeError::Parser(msg))
        }
    }

    fn get_word(&mut self) -> RuntimeResult<Word> {
        if let AnyItem::Word(word) = self.next() {
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
    symbols: HashMap<String, SymbolTag>,
    fmap: FuncMap,
}

impl Parser {
    pub fn new() -> Self {
        let fmap = crate::hashmap![
            "random".to_string() => FuncDefinition::new(true, 1, NodeList::new())
        ];

        Self {
            symbols: HashMap::new(),
            fmap,
        }
    }

    pub fn go(&mut self, input: &[AnyItem]) -> RuntimeResult<ParserOutput> {
        let mut iter = ListIter::new(input);
        let list = self.parse(&mut iter)?;
        Ok(ParserOutput::new(list, self.fmap.to_owned()))
    }

    fn parse(&mut self, iter: &mut ListIter) -> RuntimeResult<NodeList> {
        let mut list = NodeList::new();

        while !iter.is_empty() {
            let word = iter.get_word()?;
            let name = word.name();

            match name.to_lowercase().as_str() {
                "bk" | "backward" => {
                    let node = self.parse_backward(iter)?;
                    list.push(node);
                }

                "clean" => {
                    list.push(self.parse_clean());
                }

                "cs" | "clearscreen" => {
                    list.push(self.parse_clear_screen());
                }

                "fd" | "forward" => {
                    let node = self.parse_forward(iter)?;
                    list.push(node);
                }

                "fn" => {
                    self.parse_fn(iter)?;
                }

                "home" => {
                    list.push(self.parse_home());
                }

                "let" => {
                    let node = self.parse_let(iter)?;
                    list.push(node);
                }

                "lt" | "left" => {
                    let node = self.parse_left(iter)?;
                    list.push(node);
                }

                "pd" | "pendown" => {
                    list.push(self.parse_pen_down());
                }

                "pu" | "penup" => {
                    list.push(self.parse_pen_up());
                }

                "repeat" => {
                    let node = self.parse_repeat(iter)?;
                    list.push(node);
                }

                "rt" | "right" => {
                    let node = self.parse_right(iter)?;
                    list.push(node);
                }

                "seth" | "setheading" => {
                    let node = self.parse_set_heading(iter)?;
                    list.push(node);
                }

                "setpc" | "setpencolor" => {
                    let node = self.parse_set_pen_color(iter)?;
                    list.push(node);
                }

                "setpos" => {
                    let node = self.parse_set_pos(iter)?;
                    list.push(node);
                }

                "setsc" | "setscreencolor" => {
                    let node = self.parse_set_screen_color(iter)?;
                    list.push(node);
                }

                "setxy" => {
                    let node = self.parse_setxy(iter)?;
                    list.push(node);
                }

                "setx" => {
                    let node = self.parse_setx(iter)?;
                    list.push(node);
                }

                "sety" => {
                    let node = self.parse_sety(iter)?;
                    list.push(node);
                }

                _ => match self.symbols.get(name) {
                    Some(SymbolTag::Func) => {
                        let node = self.parse_call(iter, word);
                        list.push(node);
                    }
                    Some(SymbolTag::Var) => {
                        let node = self.parse_assign(iter, word)?;
                        list.push(node);
                    }
                    _ => {
                        let msg = format!("unrecognized symbol {}", name);
                        return Err(RuntimeError::Parser(msg));
                    }
                },
            }
        }

        Ok(list)
    }

    fn parse_assign(&mut self, iter: &mut ListIter, name: Word) -> RuntimeResult<Node> {
        iter.expect(2)?;
        iter.get_assignment()?;
        let rhs = iter.get_expression()?;
        let node = AssignNode::new(name.name().to_string(), rhs);
        Ok(Node::Assign(node))
    }

    fn parse_backward(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let distance = iter.get_expression()?;
        let move_node = MoveNode::new(distance, Direction::Backward);
        Ok(Node::Move(move_node))
    }

    fn parse_call(&mut self, _: &mut ListIter, name: Word) -> Node {
        let call_node = CallNode::new(name);
        Node::Call(call_node)
    }

    fn parse_clean(&mut self) -> Node {
        Node::Clean
    }

    fn parse_clear_screen(&mut self) -> Node {
        Node::ClearScreen
    }

    fn parse_fn(&mut self, iter: &mut ListIter) -> RuntimeResult {
        iter.expect(2)?;
        let name = iter.get_word()?;
        self.check_symbol(name.name(), SymbolTag::Func)?;
        let block = iter.get_block()?;
        let mut block_iter = ListIter::new(&block);
        let list = self.parse(&mut block_iter)?;
        let func = FuncDefinition::new(false, 0, list);
        self.fmap.insert(name.name().to_string(), func);
        Ok(())
    }

    fn parse_forward(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let distance = iter.get_expression()?;
        let move_node = MoveNode::new(distance, Direction::Forward);
        Ok(Node::Move(move_node))
    }

    fn parse_home(&mut self) -> Node {
        Node::Home
    }

    fn parse_let(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(3)?;
        let var = iter.get_word()?;
        self.check_symbol(var.name(), SymbolTag::Var)?;
        iter.get_assignment()?;
        let rhs = iter.get_expression()?;
        let l_node = LetNode::new(var.name().to_string(), rhs);
        Ok(Node::Let(l_node))
    }

    fn parse_left(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let rotate_node = RotateNode::new(angle, Direction::Left);
        Ok(Node::Rotate(rotate_node))
    }

    fn parse_pen_down(&mut self) -> Node {
        let pen_node = PenNode::Down;
        Node::Pen(pen_node)
    }

    fn parse_pen_up(&mut self) -> Node {
        let pen_node = PenNode::Up;
        Node::Pen(pen_node)
    }

    fn parse_repeat(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(2)?;
        let count = iter.get_expression()?;
        let block = iter.get_block()?;
        let mut block_iter = ListIter::new(&block);
        let node_list = self.parse(&mut block_iter)?;
        let repeat_node = RepeatNode::new(count, node_list);
        Ok(Node::Repeat(repeat_node))
    }

    fn parse_right(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let rotate_node = RotateNode::new(angle, Direction::Right);
        Ok(Node::Rotate(rotate_node))
    }

    fn parse_set_heading(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let angle = iter.get_expression()?;
        let node = SetHeadingNode::new(angle);
        Ok(Node::SetHeading(node))
    }

    fn parse_set_pen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let color = iter.get_list_num_word()?;
        let pen_color_node = SetPenColorNode::new(color);
        Ok(Node::SetPenColor(pen_color_node))
    }

    fn parse_set_pos(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let pos = iter.get_list()?;
        let mut pos_iter = ListIter::new(&pos);
        self.parse_setxy(&mut pos_iter)
    }

    fn parse_set_screen_color(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let color = iter.get_list_num_word()?;
        let pen_color_node = SetScreenColorNode::new(color);
        Ok(Node::SetScreenColor(pen_color_node))
    }

    fn parse_setxy(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(2)?;
        let x = iter.get_expression()?;
        let y = iter.get_expression()?;
        let pos_node = SetPositionNode::new(Some(x), Some(y));
        Ok(Node::SetPosition(pos_node))
    }

    fn parse_setx(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let x = iter.get_expression()?;
        let pos_node = SetPositionNode::new(Some(x), None);
        Ok(Node::SetPosition(pos_node))
    }

    fn parse_sety(&mut self, iter: &mut ListIter) -> RuntimeResult<Node> {
        iter.expect(1)?;
        let y = iter.get_expression()?;
        let pos_node = SetPositionNode::new(None, Some(y));
        Ok(Node::SetPosition(pos_node))
    }

    fn check_symbol(&mut self, name: &str, tag: SymbolTag) -> RuntimeResult {
        if !self.symbols.contains_key(name) {
            self.symbols.insert(name.to_string(), tag);
            Ok(())
        } else {
            let msg = format!("duplicate symbol {}", name);
            Err(RuntimeError::Parser(msg))
        }
    }
}
