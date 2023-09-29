use assemble_tree_poc::{assemble_tree, Node};
pub fn main() {
    let _node: Node = Node::new("a");
    let _node: Node = Node::new("a");
    let _node: Node = {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        base
    };
    let _node: Node = {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        base
    };
}
