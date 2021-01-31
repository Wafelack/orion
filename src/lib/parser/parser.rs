use crate::{parser::node::{Node, NodeType}, lexer::tokens::Token};

pub struct Parser {
    tokens: Vec<Token>,
    ast: Node,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ast: Node::new(NodeType::Scope),
            current: 0,
        }
    }
    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    fn peek(&self) -> Token {
        if self.is_at_end() {
            return Token::RightParen;
        }
        self.tokens[self.current].clone()
    }
    fn parse_block(&mut self) -> crate::Result<Node> {

        let mut current = self.advance();

        let mut toret = match current {
            Token::Identifier(s) => Node::new(NodeType::FunctionCall(s)),
            _ => return Err(crate::error!("Unexpected token:", current)),
        };

        while !self.is_at_end() && self.peek() != Token::RightParen {
            current = self.advance();

            match current {
                Token::String(s) => toret.add_child(Node::new(NodeType::String(s))),
                Token::Int(i) => toret.add_child(Node::new(NodeType::Int(i))),
                Token::Float(f) => toret.add_child(Node::new(NodeType::Float(f))),
                Token::Bool(b) => toret.add_child(Node::new(NodeType::Bool(b))),
                Token::LeftParen => toret.add_child(self.parse_block()?),
                Token::Nil => toret.add_child(Node::new(NodeType::Nil)),
                Token::Identifier(i) => toret.add_child(Node::new(NodeType::Identifier(i))),
                Token::LeftBrace => toret.add_child(self.parse_scope()?),
                _ => return Err(crate::error!("Unexpected token:", current))
            }
        }

        if self.peek() == Token::RightParen {
            self.advance();
        }

        Ok(toret)
    }

    fn parse_scope(&mut self) -> crate::Result<Node> {
        let mut toret = Node::new(NodeType::Scope);

        let mut current = Token::Nil;


        while !self.is_at_end() && self.peek() != Token::RightBrace {
            current = self.advance();

            match current {
                Token::String(s) => toret.add_child(Node::new(NodeType::String(s))),
                Token::Int(i) => toret.add_child(Node::new(NodeType::Int(i))),
                Token::Float(f) => toret.add_child(Node::new(NodeType::Float(f))),
                Token::Nil => toret.add_child(Node::new(NodeType::Nil)),
                Token::Bool(b) => toret.add_child(Node::new(NodeType::Bool(b))),
                Token::LeftParen => toret.add_child(self.parse_block()?),
                Token::Identifier(i) => toret.add_child(Node::new(NodeType::Identifier(i))),
                Token::LeftBrace => toret.add_child(self.parse_scope()?),
                _ => return Err(crate::error!("Unexpected token:", current))
            }
        }
        if self.peek() == Token::RightBrace {
            self.advance();
        }

        Ok(toret)
    }

    fn parse(&mut self) -> crate::Result<()> {
        let token = self.advance();
        match token {
            Token::LeftParen => {
                let to_add = self.parse_block()?;
                self.ast.add_child(to_add);
                Ok(())
            },
            Token::LeftBrace => {
                let to_add = self.parse_scope()?;
                self.ast.add_child(to_add);
                Ok(())
            },
            x => {
                Err(crate::error!("Unexpected token:", x))
            },
        }
        
    }


    pub fn parse_tokens(&mut self) -> crate::Result<Node> {
        while !self.is_at_end() {
            self.parse()?;
        }
        Ok(self.ast.clone())
    }
}