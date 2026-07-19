use crate::tree::Node;

#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Node>,
}

impl Stack {
    pub fn pop(&mut self) -> Option<Node> {
        match self.values.clone().first() {
            Some(node) => {
                match self.values.get(1..) {
                    Some(slice) => self.values = slice.to_vec(),
                    None => {}
                };
                Some(node.clone())
            }
            None => None,
        }
    }

    pub fn push(&mut self, node: Node) {
        self.values.extend(Vec::from([node]));
    }

    pub const fn len(&self) -> usize {
        self.values.len()
    }

    pub fn sort(&mut self) {
        self.values.sort_by_key(|k| k.frequency);
    }
}
