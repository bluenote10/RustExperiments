use debug_rust_analyzer_unused_variable_macro::{assemble_tree, Node};
pub fn main() {
    let _node: Node = {
        let base = Node::new("a");
        base.add_child(Node::new("b"));
        base.add_child(Node::new("c"));
        base
    };
    let _node: Node = Node::new("a");
    let _node: Node = Node::new("a");
}
