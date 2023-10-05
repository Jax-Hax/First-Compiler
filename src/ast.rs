use crate::lexer::Token;
use std::vec::IntoIter;

pub struct AST {
    tokens: IntoIter<Token>,
    cur_token: Token,
}
impl AST {
    pub fn new(mut tokens: IntoIter<Token>) -> Self {
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
        self.next_tok(); //eat identifier
        match self.cur_token {
            Token::Other(char) => {
                if char != '(' {
                    return Node::Variable(identifier);
                }
            }
            _ => {}
        }
        self.next_tok(); //eat (
        let mut args = vec![];
        match self.cur_token {
            Token::Other(char) => {
                if char != ')' {
                    loop {
                        let node = self.parse_expression();
                        if node.is_some() {
                            args.push(Box::new(node.unwrap()));
                        } else {
                            return Node::Error("Failed to parse expression".to_string());
                        }
                        match self.cur_token {
                            Token::Other(char) => {
                                if char == ')' {
                                    break;
                                } else if char != ',' {
                                    return Node::Error(
                                        "Expected ')' or ',' in an argument list".to_string(),
                                    );
                                }
                            }
                            _ => {}
                        }
                        self.next_tok();
                    }
                }
            }
            _ => {}
        }
        self.next_tok(); // eat )
        return Node::FunctionCall {
            name: identifier,
            args,
        };
    }
    fn parse_parenthesis(&mut self) -> Node {
        self.next_tok(); //consume '('
        let node = self.parse_expression();
        if node.is_none() {
            return Node::Error("Expression is invalid".to_string());
        }
        match self.cur_token {
            Token::Other(char) => {
                if char != ')' {
                    return Node::Error("Expected ')' but got {char}".to_string());
                }
            }
            _ => {}
        }
        self.next_tok(); //consume ')'
        return node.unwrap();
    }
    fn parse_expression(&mut self) -> Option<Node> {
      let lhs = self.parse_primary();
    }
    fn parse_primary(&mut self) -> Node {
        match &self.cur_token {
            Token::Number(num) => parse_num(*num),
            Token::Identifier(identifier) => self.parse_identifier(identifier.to_string()),
            Token::Other(char) => {
                if *char == '(' {
                    self.parse_parenthesis()
                } else {
                    Node::Error(format!("Unknown character found: {}", char))
                }
            }
            _ => Node::Error(format!(
                "Unknown token found when parsing expression: {:#?}",
                self.cur_token
            )),
        }
    }
}
pub fn construct_tree(tokens: IntoIter<Token>) -> Option<Vec<Node>> {
    let mut ast = AST::new(tokens);
    let mut nodes = vec![];
    loop {
        let node = ast.parse_primary();
        ast.next_tok();
        println!("{:#?}", node);
        match node {
            Node::Eof => return Some(nodes),
            Node::Error(err) => {
                eprintln!("Error: {}", err);
                return None;
            }
            _ => {}
        }
        if let Node::Eof = node {
            return Some(nodes);
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
    Error(String),
}
#[derive(Debug)]
pub struct FunctionParams {
    pub name: String,
    pub args: Vec<String>,
}
