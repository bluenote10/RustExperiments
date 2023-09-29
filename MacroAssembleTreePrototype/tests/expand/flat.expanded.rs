use assemble_tree_poc::{assemble_tree, Node};
pub fn main() {
    {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        base.add_child(Node::new("c"));
        base.add_child(Node::new("d"));
        base
    };
    {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        base.add_child(Node::new("c"));
        base.add_child(Node::new("d"));
        base
    };
}
