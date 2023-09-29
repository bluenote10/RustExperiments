#![allow(dead_code, unused_imports)]

use assemble_tree_poc::{assemble_tree, Node};

pub fn main() {
    assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
                Node::new("c") => {
                    Node::new("d")
                }
            }
        }
    );
}
