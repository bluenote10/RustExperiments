use debug_rust_analyzer_unused_variable_macro::{assemble_tree, Node};

pub fn main() {
    assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
                Node::new("c") => {
                    Node::new("d")
                }
            }
        }
    );
}
