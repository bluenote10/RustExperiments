#![allow(dead_code, unused_imports)]
use assemble_tree_poc::{assemble_tree, Node};
pub fn main() {
    let a = Node::new("a");
    let b = Node::new("b");
    let c = Node::new("c");
    {
        let base = a;
        let child = {
            let base = b;
            base.add_child(c);
            base
        };
        base.add_child(child);
        base
    };
}
fn is_not_moved(_n: Node) {}
