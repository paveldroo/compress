use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    value: Option<char>,
    pub frequency: i32,
    right: Option<Box<Self>>,
    left: Option<Box<Self>>,
}

pub fn get_tree(chars_map: HashMap<char, i32>) -> Option<Node> {
    let nodes = get_nodes_vec(chars_map);
    build_tree(nodes)
}

fn get_nodes_vec(chars_map: HashMap<char, i32>) -> Vec<Node> {
    let mut nodes = Vec::new();
    for (ch, freq) in chars_map {
        let node = Node {
            value: Some(ch),
            frequency: freq,
            right: None,
            left: None,
        };
        nodes.push(node);
    }
    nodes
}

fn build_tree(nodes_vec: Vec<Node>) -> Option<Node> {
    let mut stack = nodes_vec;
    stack.sort_by_key(|k| -k.frequency);

    while stack.len() > 1 {
        let node1 = stack.pop()?;
        let node2 = stack.pop()?;

        let node = Node {
            value: None,
            frequency: node1.frequency + node2.frequency,
            right: Some(Box::new(node2)),
            left: Some(Box::new(node1)),
        };

        stack.push(node);
        stack.sort_by_key(|n| -n.frequency);
    }
    stack.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_tree() {
        let nodes = Vec::from([
            Node {
                value: Some('b'),
                frequency: 1,
                right: None,
                left: None,
            },
            Node {
                value: Some('c'),
                frequency: 22,
                right: None,
                left: None,
            },
            Node {
                value: Some('a'),
                frequency: 100,
                right: None,
                left: None,
            },
        ]);

        build_tree(nodes);
    }
}
