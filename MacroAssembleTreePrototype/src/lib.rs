#[derive(Clone)]
pub struct Node;

impl Node {
    pub fn new(_s: &'static str) -> Self {
        Node {}
    }
    pub fn add_child(&self, _child: Node) {
        unimplemented!()
    }
    pub fn share(&self) -> Self {
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
            // let base = &$base;
            // assemble_tree!( @iter_children, base, $($other)*);
            // base.share()
            // let base = $base.share();
            // assemble_tree!( @iter_children, base, $($other)*);
            // base
        }
    };

    // Patterns for '.. children' syntax
    (@iter_children, $base:expr, .. $child_iter:expr $(,)?) => {
        for child in $child_iter {
            $base.add_child(child);
        }
    };
    (@iter_children, $base:expr, .. $child_iter:expr, $($other:tt)+) => {
        for child in $child_iter {
            $base.add_child(child);
        }
        assemble_tree!( @iter_children, $base, $($other)*)
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
        let child = assemble_tree!( $child => { $($children)* }); // True recursion
        $base.add_child(child);
    };
    (@iter_children, $base:expr, $child:expr => { $($children:tt)+ }, $($other:tt)+) => {
        let child = assemble_tree!( $child => { $($children)* }); // True recursion
        $base.add_child(child);
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
