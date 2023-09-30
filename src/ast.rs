use std::vec::IntoIter;

use crate::lexer::Token;

pub fn construct_tree(mut tokens: IntoIter<Token>) {
    let token = tokens.next().unwrap();
}