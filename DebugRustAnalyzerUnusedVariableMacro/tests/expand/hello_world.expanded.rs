use debug_rust_analyzer_unused_variable_macro::assemble_tree;
pub fn main() {
    {
        ::std::io::_print(format_args!("Hello world\n"));
    };
    let temp = Node::new("b");
    let temp = Node::new("c");
    temp.add_child(Node::new("d"));
    temp.add_child(temp);
    Node::new("a").add_child(temp);
}
