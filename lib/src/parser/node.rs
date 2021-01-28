pub struct Node {
    ntype: NodeType,
    children: Vec<Node>,
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

enum NodeType {
    Block,
    Scope,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),

}