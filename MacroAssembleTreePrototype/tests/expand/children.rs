#![allow(dead_code, unused_imports)]

use assemble_tree_poc::{assemble_tree, Node};

pub fn main() {
    let children = Vec::<Node>::new();
    let _node: Node = assemble_tree!(
        Node::new("a") => {
            .. children
        }
    );

    let children = Vec::<Node>::new();
    let _node: Node = assemble_tree!(
        Node::new("a") => {
            .. children,
        }
    );

    let children = Vec::<Node>::new();
    let _node: Node = assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
                .. children,
            },
        }
    );

    let children_a = Vec::<Node>::new();
    let children_b = Vec::<Node>::new();
    let _node: Node = assemble_tree!(
        Node::new("a") => {
            .. children_a,
            .. children_b,
        }
    );

    let children_a = Vec::<Node>::new();
    let children_b = Vec::<Node>::new();
    let children_c = Vec::<Node>::new();
    let _node: Node = assemble_tree!(
        Node::new("a") => {
            .. children_a.clone(),
            .. children_a,
            Node::new("b") => {
                .. children_b.clone(),
                .. children_b,
            },
            Node::new("c") => {
                .. children_c.clone(),
                .. children_c,
            }
        }
    );
}
