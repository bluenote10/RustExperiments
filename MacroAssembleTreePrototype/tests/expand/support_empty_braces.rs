use assemble_tree_poc::{assemble_tree, Node};

pub fn main() {
    let _node: Node = assemble_tree!(
        Node::new("a") => {
        },
    );

    let _node: Node = assemble_tree!(
        Node::new("a") => {
        }
    );

    let _node: Node = assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
            },
        }
    );

    let _node: Node = assemble_tree!(
        Node::new("a") => {
            Node::new("b") => {
            }
        }
    );
}
