use debug_rust_analyzer_unused_variable_macro::{assemble_tree, Node};
pub fn main() {
    {
        let base = Node::new("a");
        {
            let base = Node::new("b");
            {
                let base = Node::new("c");
                base.add_child(Node::new("d"));
                base
            };
            base.add_child(Node::new("c"));
            base
        };
        base.add_child(Node::new("b"));
        base
    };
}
