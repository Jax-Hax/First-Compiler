use std::vec::IntoIter;

use crate::lexer::Token;

pub fn construct_tree(mut tokens: IntoIter<Token>) {
    let mut token = tokens.next().unwrap();
    loop {
        
        token = tokens.next().unwrap();
    }
}
fn match_token(cur_token: &mut Token, tokens: &mut IntoIter<Token>) -> NodeReturn {
    match cur_token {
        Token::Eof => NodeReturn::Eof,
        Token::Fn => todo!(),
        Token::Identifier(_) => todo!(),
        Token::Number(num) => NodeReturn::Node(Node::Val(num)),
        Token::Other(char) => {
            if char == &mut '(' {

            }
        },
    }
}
fn match_char(char: &mut char, tokens: &mut IntoIter<Token>) {
    if char == &mut '(' {

    }
}
fn parse_parenthesis(tokens: &mut IntoIter<Token>) {
    let token = tokens.next().unwrap();
}
fn parse_expr() {

}
fn parse_identifier_expr(identifier: String, tokens: &mut IntoIter<Token>) -> Node{ //called when the current token is an identifier token
    let mut token = tokens.next().unwrap(); //eat identifier
    match token {
        Token::Other(char) => if char != '(' {
            return Node::Variable(identifier)
        }
        _ => {}
    }
    token = tokens.next().unwrap(); //eat (
    let args = vec![];
    match token {
        Token::Other(char) => if char != ')' {
            loop {
                if 
            }
        }
    }
    Node::FunctionCall { name: identifier, args: () }
}
enum NodeReturn{
    Node(Node),
    Eof,
    Nothing,
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
        body: Box<Node>,
    },
    Variable(String),
}
pub struct FunctionParams {
    pub name: String,
    pub args: Vec<String>,
}
