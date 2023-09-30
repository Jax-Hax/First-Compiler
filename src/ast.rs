use std::vec::IntoIter;

use crate::lexer::Token;

pub fn construct_tree(mut tokens: IntoIter<Token>) {
    let token = tokens.next().unwrap();
}
pub enum Node {
    Val(f32),
    UnaryExpr {
        op: char,
        child: Box<Node>,
    },
    BinaryExpr {
        op: char,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    FunctionCall {
        name: String,
        args: Vec<Box<Node>>,
    },
    Function {
        params: FunctionParams,
        args: Vec<Box<Node>>,
    },
    Variable(String),
}
pub struct FunctionParams {
    pub name: String,
    pub args: Vec<String>,
}
