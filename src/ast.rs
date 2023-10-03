use std::vec::IntoIter;
use crate::lexer::Token;

pub struct AST {
  tokens: IntoIter<Token>,
  cur_token: Token,
}
impl AST {
  pub fn new(mut tokens: IntoIter<Token>) -> Self{
    let token = tokens.next().unwrap();
    Self {
      cur_token: token,
      tokens,
    }
  }
  pub fn next_tok(&mut self) {
    let token = self.tokens.next().unwrap();
    self.cur_token = token;
  }
  fn parse_identifier(&mut self, identifier: String) -> Node {
    self.next_tok();
    if let Token::Other('(') = self.cur_token {
      return Node::Variable(identifier);
    }
    self.next_tok();
    //let args = vec![];
    if let Token::Other(')') = self.cur_token {
      
    }
    return Node::Val(1.);
  }
  fn parse_primary(&mut self) -> Node {
    match &self.cur_token{
      Token::Number(num) => parse_num(*num),
      Token::Identifier(identifier) => self.parse_identifier(identifier.to_string()),
      _ => {eprintln!("Unknown token found when parsing expression: {:#?}", self.cur_token); return Node::Val(1.0)}
    }
  }
}
pub fn construct_tree(tokens: IntoIter<Token>) -> Vec<Node> {
  let mut ast = AST::new(tokens);
  let mut nodes = vec![];
  loop {
    let node = ast.parse_primary();
    ast.next_tok();
    println!("{:#?}", node);
    if let Node::Eof = node {
      return nodes;
    }
    nodes.push(node);
  }
}
fn parse_num(num: f32) -> Node {
    Node::Val(num)
}
#[derive(Debug)]
pub enum Node {
    Val(f32),
    Eof,
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
#[derive(Debug)]
pub struct FunctionParams {
    pub name: String,
    pub args: Vec<String>,
}
