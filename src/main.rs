mod lexer;
use lexer::get_tok;
use std::fs;
fn main() {
  let data = fs::read_to_string("code.txt").expect("Unable to read file");
  let mut chars = data.chars();
  loop{
  println!("{:#?}", get_tok(&mut chars));
  }
}