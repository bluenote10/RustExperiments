use debug_rust_analyzer_unused_variable_macro::assemble_tree;
pub fn main() {
    {
        let base = Node::new("a");
        let child = {
            let base = Node::new("b");
            let child = {
                Node::new("c").add_child(Node::new("d"));
            };
            base.add_child(child);
            base;
        };
        base.add_child(child);
        base;
    };
}
