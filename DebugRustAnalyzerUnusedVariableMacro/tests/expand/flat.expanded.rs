use debug_rust_analyzer_unused_variable_macro::{assemble_tree, Node};
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
