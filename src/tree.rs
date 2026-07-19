use std::collections::HashMap;

use crate::stack;

#[derive(Debug, Clone)]
pub struct Node {
    value: Option<char>,
    pub frequency: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

pub fn get_tree(chars_map: HashMap<char, i32>) -> Node {
    let nodes = get_sorted_nodes(chars_map);
    let tree = build_tree(nodes);
    tree
}

fn get_sorted_nodes(chars_map: HashMap<char, i32>) -> Vec<Node> {
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

    nodes.sort_by_key(|k| k.frequency);
    nodes
}

fn build_tree(sorted_nodes: Vec<Node>) -> Node {
    let mut stack = stack::Stack { values: vec![] };
    for node in sorted_nodes {
        stack.push(node);
    }

    while stack.len() > 1 {
        let node1 = stack.pop();
        let node2 = stack.pop();

        let node = Node {
            value: None,
            frequency: node1.frequency + node2.frequency,
            right: Some(Box::new(node2)),
            left: Some(Box::new(node1)),
        };

        stack.push(node);
        stack.sort();
    }

    let node = stack.pop();
    dbg!(&node);
    node
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;

//     #[test]
//     fn valid_tree() {
//         let nodes = Vec::from([
//             Node {
//                 value: Some('b'),
//                 frequency: 1,
//                 right: None,
//                 left: None,
//             },
//             Node {
//                 value: Some('c'),
//                 frequency: 22,
//                 right: None,
//                 left: None,
//             },
//             Node {
//                 value: Some('a'),
//                 frequency: 100,
//                 right: None,
//                 left: None,
//             },
//         ]);

//         build_tree(nodes);
//     }

// #[test]
// fn valid_sorting_nodes() {
//     let map = HashMap::from([('a', 100), ('b', 1), ('c', 22)]);
//     let want = Vec::from([
//         Node {
//             value: 'b',
//             frequency: 1,
//             right: None,
//             left: None,
//         },
//         Node {
//             value: 'c',
//             frequency: 22,
//             right: None,
//             left: None,
//         },
//         Node {
//             value: 'a',
//             frequency: 100,
//             right: None,
//             left: None,
//         },
//     ]);
//     let got = get_sorted_nodes(map);
//     // println!("{got:?}");
//     // assert_eq!(got.get(1).unwrap()., want, "expected: {want:?}, got: {got:?}")
// }
// }
