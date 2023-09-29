struct Node;

impl Node {
    fn new() -> Self {
        Node {}
    }
    fn add_child(&self, _child: Node) {
        unimplemented!()
    }
}

macro_rules! assemble_tree {
    ($base:expr => { $($other:tt)* }) => {
        assemble_tree!( @recurse, $base, $($other)*)
    };

    // Patterns for 'child' syntax
    (@recurse, $base:expr, $child:expr $(,)?) => {
        $base.add_child($child)
    };
    (@recurse, $base:expr, $child:expr, $($other:tt)+) => {
        $base.add_child($child);
        assemble_tree!( @recurse, $base, $($other)*)
    };

    // Patterns for 'child => { ... }' syntax
    (@recurse, $base:expr, $child:expr => { $($children:tt)* } $(,)?) => {
        let temp = $child;
        assemble_tree!( temp => { $($children)* });
        $base.add_child(temp)
    };
    (@recurse, $base:expr, $child:expr => { $($children:tt)* }, $($other:tt)+) => {
        let temp = $child;
        assemble_tree!( temp => { $($children)* });
        $base.add_child(temp);
        assemble_tree!( @recurse, $base, $($other)*)
    };
}

fn main() {
    assemble_tree!(
        Node::new() => {
            Node::new() => {
                Node::new()
            }
        }
    );

    assemble_tree!(
        Node::new() => {
            Node::new() => {
                Node::new() => {
                    Node::new()
                }
            }
        }
    );
}
