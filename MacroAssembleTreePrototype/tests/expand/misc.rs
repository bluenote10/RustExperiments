#![allow(dead_code, unused_imports)]

use assemble_tree_poc::{assemble_tree, Node};

pub fn main() {
    assemble_tree!(
        Node::new("a") => {
            Node::new("b") => { Node::new("sub") },
            Node::new("c"),
        }
    );

    assemble_tree!(
        Node::new("a") => {
            Node::new("b"),
            Node::new("c") => { Node::new("sub") },
        }
    );

    assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
                Node::new("c"),
            },
            Node::new("d") => {
                Node::new("e"),
            },
            Node::new("f") => {
                Node::new("g"),
            },
        },
    );

    assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
                Node::new("c") => {
                    Node::new("d")
                }
            },
            Node::new("e") => {
                Node::new("f") => {
                    Node::new("g")
                }
            }
        }
    );
}
