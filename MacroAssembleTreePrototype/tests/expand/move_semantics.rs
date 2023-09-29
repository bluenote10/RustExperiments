#![allow(dead_code, unused_imports)]

use assemble_tree_poc::{assemble_tree, Node};

pub fn main() {
    let a = Node::new("a");
    let b = Node::new("b");
    let c = Node::new("c");
    assemble_tree!(
        a => {
            b => {
                c
            }
        }
    );

    // They are all moved -- consistent.
    // is_not_moved(a);
    // is_not_moved(b);
    // is_not_moved(c);
}

fn is_not_moved(_n: Node) {}
