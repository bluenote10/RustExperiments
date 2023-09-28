use debug_rust_analyzer_unused_variable_macro::assemble_tree;
pub fn main() {
    {
        Node::new("a").add_child(Node::new("b"));
        Node::new("a").add_child(Node::new("c"));
        Node::new("a").add_child(Node::new("d"));
    };
}
