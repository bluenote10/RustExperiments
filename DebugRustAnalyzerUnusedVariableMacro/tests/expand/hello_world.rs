use debug_rust_analyzer_unused_variable_macro::assemble_tree;

pub fn main() {
    println!("Hello world");
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
