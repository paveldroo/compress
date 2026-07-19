use crate::tree::Node;

#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Node>,
}

impl Stack {
    pub fn pop(&mut self) -> Node {
        let node = self.values.get(0).cloned().unwrap();
        self.values = self.values[1..].to_vec();
        node
    }

    pub fn push(&mut self, node: Node) {
        self.values.extend(Vec::from([node]));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn sort(&mut self) {
        self.values.sort_by_key(|k| k.frequency);
    }
}
