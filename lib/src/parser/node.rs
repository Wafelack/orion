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