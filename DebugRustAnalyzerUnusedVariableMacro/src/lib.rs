pub struct Node;

impl Node {
    pub fn new(_s: &'static str) -> Self {
        Node {}
    }
    pub fn add_child(&self, _child: Node) {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! assemble_tree {
    ($base:expr => { $($other:tt)+ } $(,)?) => {
        {
            let base = $base;
            assemble_tree!( @iter_children, base, $($other)*);
            base
        }
    };

    // Patterns for 'child' syntax
    (@iter_children, $base:expr, $child:expr $(,)?) => {
        $base.add_child($child);
    };
    (@iter_children, $base:expr, $child:expr, $($other:tt)+) => {
        $base.add_child($child);
        assemble_tree!( @iter_children, $base, $($other)*)
    };

    // Patterns for 'child => { ... }' syntax
    (@iter_children, $base:expr, $child:expr => { $($children:tt)+ } $(,)?) => {
        assemble_tree!( $child => { $($children)* }); // True recursion
        $base.add_child($child);
    };
    (@iter_children, $base:expr, $child:expr => { $($children:tt)+ }, $($other:tt)+) => {
        assemble_tree!( $child => { $($children)* }); // True recursion
        $base.add_child($child);
        assemble_tree!( @iter_children, $base, $($other)*)
    };

    // Support for empty braces
    (@iter_children, $base:expr, $child:expr => {} $(,)?) => {
        $base.add_child($child);
    };
    (@iter_children, $base:expr, $child:expr => {}, $($other:tt)+) => {
        $base.add_child($child);
        assemble_tree!( @iter_children, $base, $($other)*)
    };

    // Support for empty braces at top-level
    ($base:expr => {} $(,)?) => {
        $base
    };

    // Support for single expressions
    ($base:expr $(,)?) => {
        $base
    };
}
