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
  fn parse_tok(&mut self) -> Node {
    match self.cur_token{
      
    }
  }
  fn parse_num(num: f32) -> Node {
    Node::Val(num)
  }
  fn parse_primary(&mut self) {
    match self.cur_token{
      _ => {eprintln!("Unknown token found when parsing expression: {:#?}", self.cur_token);}
    }
  }
}
pub fn construct_tree(mut tokens: IntoIter<Token>) -> Vec<Node> {
  let mut ast = AST::new(tokens);
  let mut nodes = vec![];
  loop {
    let node = ast.parse_tok();
    ast.next_tok();
    if let Node::Eof = node {
      return nodes;
    }
    nodes.push(node);
  }
}
/*fn match_token(cur_token: &mut Token, tokens: &mut IntoIter<Token>) -> NodeReturn {
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
}*/
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
