mod lexer;
use lexer::get_tok;
use std::fs;

use crate::lexer::Token;
fn main() {
  let data = fs::read_to_string("code.txt").expect("Unable to read file");
  let mut chars = data.chars();
  loop{
    let token = get_tok(&mut chars);
  println!("{:#?}", token);
  if let Token::Eof = token {
    break;
  }
  }
}