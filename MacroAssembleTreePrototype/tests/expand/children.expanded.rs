#![allow(dead_code, unused_imports)]
use assemble_tree_poc::{assemble_tree, Node};
pub fn main() {
    let children = Vec::<Node>::new();
    let _node: Node = {
        let base = Node::new("a");
        for child in children {
            base.add_child(child)
        }
        base
    };
    let children = Vec::<Node>::new();
    let _node: Node = {
        let base = Node::new("a");
        for child in children {
            base.add_child(child)
        }
        base
    };
    let children = Vec::<Node>::new();
    let _node: Node = {
        let base = Node::new("a");
        let child = {
            let base = Node::new("b");
            for child in children {
                base.add_child(child)
            }
            base
        };
        base.add_child(child);
        base
    };
    let children_a = Vec::<Node>::new();
    let children_b = Vec::<Node>::new();
    let _node: Node = {
        let base = Node::new("a");
        for child in children_a {
            base.add_child(child);
        }
        for child in children_b {
            base.add_child(child)
        }
        base
    };
    let children_a = Vec::<Node>::new();
    let children_b = Vec::<Node>::new();
    let children_c = Vec::<Node>::new();
    let _node: Node = {
        let base = Node::new("a");
        for child in children_a.clone() {
            base.add_child(child);
        }
        for child in children_a {
            base.add_child(child);
        }
        let child = {
            let base = Node::new("b");
            for child in children_b.clone() {
                base.add_child(child);
            }
            for child in children_b {
                base.add_child(child)
            }
            base
        };
        base.add_child(child);
        let child = {
            let base = Node::new("c");
            for child in children_c.clone() {
                base.add_child(child);
            }
            for child in children_c {
                base.add_child(child)
            }
            base
        };
        base.add_child(child);
        base
    };
}
