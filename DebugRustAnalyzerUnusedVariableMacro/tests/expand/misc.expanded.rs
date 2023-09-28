use debug_rust_analyzer_unused_variable_macro::{assemble_tree, Node};
pub fn main() {
    {
        let base = Node::new("a");
        let child = {
            let base = Node::new("b");
            base.add_child(Node::new("sub"));
            base
        };
        base.add_child(child);
        base.add_child(Node::new("c"));
        base
    };
    {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        let child = {
            let base = Node::new("c");
            base.add_child(Node::new("sub"));
            base
        };
        base.add_child(child);
        base
    };
    {
        let base = Node::new("a");
        let child = {
            let base = Node::new("b");
            base.add_child(Node::new("c"));
            base
        };
        base.add_child(child);
        let child = {
            let base = Node::new("d");
            base.add_child(Node::new("e"));
            base
        };
        base.add_child(child);
        let child = {
            let base = Node::new("f");
            base.add_child(Node::new("g"));
            base
        };
        base.add_child(child);
        base
    };
    {
        let base = Node::new("a");
        let child = {
            let base = Node::new("b");
            let child = {
                let base = Node::new("c");
                base.add_child(Node::new("d"));
                base
            };
            base.add_child(child);
            base
        };
        base.add_child(child);
        let child = {
            let base = Node::new("e");
            let child = {
                let base = Node::new("f");
                base.add_child(Node::new("g"));
                base
            };
            base.add_child(child);
            base
        };
        base.add_child(child);
        base
    };
}
