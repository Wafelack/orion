use crate::{Result, error, OrionError, lexer::{Token, TType}};

#[derive(Clone, Debug, PartialEq)]
pub enum NType {
    Ident(String),
    Integer(i32),
    Single(f32),
    Str(String),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn stringify(&self, indentations: usize) -> String {
        let mut toret = String::new();
        toret.push_str(&format!("\n{}[\n", gen_indents(indentations)));
        for children in &self.children {
            toret.push_str(&format!("{}{{\n", gen_indents(indentations + 1)));
            toret.push_str(&format!("{}@type : ", gen_indents(indentations + 2)));
            toret.push_str(&format!("{:?},\n", children.ntype));

            if !children.children.is_empty() {
                toret.push_str(&format!("{}@children : ", gen_indents(indentations + 2)));
                toret.push_str(&children.stringify(indentations + 3));
            }
            toret.push_str(&format!("{}}},\n", gen_indents(indentations + 1)))
        }
        toret.push_str(&format!("{}],\n", gen_indents(indentations - 1)));
        toret
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
        let raw_tp = self.advance().unwrap().clone();
        let mut to_push = match &raw_tp.ttype {
            TType::LParen => self.proc_fn()?,
            TType::Ident(i) => Node::new(NType::Ident(i.to_string())),
            _ => return error!("{}:{} | Expected Opening Parenthese or Function Call, found {}.", raw_tp.line, raw_tp.col, raw_tp.get_type())
        };

        let mut closed = false;

        while !self.is_at_end() {
            let raw = self.advance().unwrap();

            match &raw.ttype {
                TType::LParen => to_push.add_child(self.proc_fn()?),
                TType::Str(s) => to_push.add_child(Node::new(NType::Str(s.to_string()))),
                TType::Number(n) => to_push.add_child(Node::new(NType::Integer(*n))),
                TType::Float(r) => to_push.add_child(Node::new(NType::Single(*r))),
                TType::Bool(b) => to_push.add_child(Node::new(NType::Bool(*b))),
                TType::Ident(i) => to_push.add_child(Node::new(NType::Ident(i.to_string()))),
                TType::RParen => {
                    closed = true;
                    break;
                }
            }


        }

        if !closed && self.input[self.current - 1].ttype != TType::RParen {
            return error!("{}:{} | Unclosed expression, expected ')'", raw_tp.line, raw_tp.col);
        }

        Ok(to_push)
    }

    fn parse_tokens(&mut self) -> Result<()> {
        let tok = self.advance().unwrap();

        match tok.ttype {
            TType::LParen => {
                let to_add = self.proc_fn()?;
                self.output.add_child(to_add);
                Ok(())
            }

            _ => error!("{}:{} | Expected Opening Parenthese, found {}.", tok.line, tok.col, tok.get_type()),
        }
    }

    pub fn parse(&mut self) -> Result<Node> {
        while !self.is_at_end() {
            self.parse_tokens()?;
        }

        Ok(self.output.clone())
    }
}


fn gen_indents(amount: usize) -> String {
    let mut toret = String::new();
    for _ in 0..amount {
        toret.push_str("  ");
    }
    toret
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.stringify(1))?;
        Ok(())
    }
}
