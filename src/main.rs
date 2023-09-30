mod lexer;
mod ast;
use ast::construct_tree;
use lexer::get_tok;
use std::fs;

use crate::lexer::Token;
fn main() {
  let data = fs::read_to_string("code.txt").expect("Unable to read file");
  let mut chars = data.chars();
  let mut tokens = vec![];
  loop{
    let token = get_tok(&mut chars);
    tokens.push(token);
    if let Token::Eof = tokens.last().unwrap() {
      break;
    }
  }
  construct_tree(tokens.into_iter());
}