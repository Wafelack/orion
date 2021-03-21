use crate::{Result, error, OrionError, lexer::{Token, TType}};

pub enum NType {
    Ident(String),
    Integer(i32),
    Single(f32),
    Str(String),
    Bool(bool),
}

pub struct Node {
    pub ntype: NType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(ntype: NType) -> Self {
        Self {
            ntype,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

pub struct Parser {
    current: usize,
    input: Vec<Token>,
    output: Node,
}

impl Parser {
    pub fn new(input: Vec<Token>) ->  Self {
        Self {
            current: 0,
            input,
            output: Node::new(NType::Ident("begin".to_owned())),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        Some(&self.input[self.current - 1])
    }

    fn proc_fn(&mut self) -> Result<Node> {
        let raw_tp = self.advance().unwrap();
        let mut to_push = match &raw_tp.ttype {
            TType::LParen => self.proc_fn()?,
            TType::Ident(i) => Node::new(NType::Ident(i.to_string())),
            _ => return error!("{}:{} | Expected Opening Parenthese or Function Call, found {}.", raw_tp.line, raw_tp.col, raw_tp.get_type())
        };

        let mut closed = false;

        while !self.is_at_end() {
            let raw = self.advance().unwrap();

            if raw.ttype == TType::RParen {
                closed = true;
                break;
            }

            match raw.ttype {

            }


        }

        Ok(self.output.clone())
    }
}
