// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8921bcc0302b066ef5e39bc6e9a8b7e3

#![allow(dead_code)]

/*
Links:
- https://www.google.com/search?client=ubuntu&channel=fs&q=rust+borrow+self+during+iteration&ie=utf-8&oe=utf-8
- https://users.rust-lang.org/t/blog-post-series-after-nll-whats-next-for-borrowing-and-lifetimes/21864
- http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/
*/

use std::collections::HashMap;

struct Nested {
    buffers: HashMap<i32, Vec<f64>>,
}

struct Test {
    buffers: HashMap<i32, Vec<f64>>,
    nested: Nested,
    counter: i32,
}


impl Test {

    fn helper(&self) {

    }

    fn test(&mut self) {
        for (id, buffer) in self.buffers.iter_mut() {
            self.counter += 1;
        }

        /*
        for (id, buffer) in self.buffers.iter_mut() {
            self.helper();
        }
        */

        // nesting doesn't help
        /*
        let buffers = &mut self.nested.buffers;
        for (id, buffer) in buffers.iter_mut() {
            self.helper();
        }
        */
        Test::static_func(&mut self.buffers, &mut self.counter);
    }

    fn static_func(buffers: &mut HashMap<i32, Vec<f64>>, counter: &mut i32) {
        for (id, buffer) in buffers.iter_mut() {
            *counter += 1;
        }
    }
}
