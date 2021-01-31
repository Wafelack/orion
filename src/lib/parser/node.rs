#[derive(Clone,PartialEq, Debug)]
pub struct Node {
    pub ntype: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(ntype: NodeType) -> Self {
        Self {
            ntype,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum NodeType {
    Scope,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    FunctionCall(String),
    Identifier(String),
    Nil,
}


impl NodeType {
    pub fn stringy_type(&self) -> String {
        match self {
            Self::Scope => "scope",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
            Self::String(_) => "string",
            Self::FunctionCall(_) => "function call",
            Self::Identifier(_) => "identifier",
            Self::Nil => "nil",
        }.to_owned()
    }
}

pub fn stringify(node: &Node, indentations: usize) -> String {
    let mut toret = String::new();
    toret.push_str("{\n");
    for children in &node.children {
      toret.push_str(&format!("{}@type : ", gen_indents(indentations)));
      toret.push_str(&format!("{:?}\n", children.ntype));
      toret.push_str(&format!("{}@children : ", gen_indents(indentations)));
      toret.push_str(&stringify(&children, indentations + 1));
    }
    toret.push_str(&format!("{}}}\n", gen_indents(indentations)));
    toret
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
      write!(f, "{}", stringify(self, 0))?;
      Ok(())
    }
  }