
pub fn profile_fn<F: Fn() -> R, R>(f: F) -> R {
    f()
}

pub fn profile_fn_mut<F: FnMut() -> R, R>(mut f: F) -> R {
    f()
}

pub fn profile_fn_once<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}


struct State;

struct MyNode {
    elements: Vec<i32>,
}


impl MyNode {

    fn test1(&mut self) {
        let mut x = 1;
        profile_fn_once(|| {
            self.immutable_helper();
            profile_fn_once(|| {
                self.immutable_helper();
                self.mutable();
                x += 1;
            });
            profile_fn_once(|| {
                self.immutable_helper();
                self.mutable();
                x += 1;
            });
            self.immutable_helper();
            x += 1;
            // std::mem::drop(self);
        });
        profile_fn_once(|| {
            self.mutable();
            x += 1;
            profile_fn_once(|| {
                self.mutable();
                x += 1;
            });
            x += 1;
            self.mutable();
        });
        profile_fn_once(|| {
            let y = &mut x;
            profile_fn_once(|| {
                self.mutable();
                *y += 1;
            });
            *y += 1;
        });
        println!("{}", x);
    }

    fn test2(&mut self) {
        for x in &self.elements {
            self.immutable_helper();
            //self.mutable();
            profile_fn_mut(|| {
                self.immutable_helper();
                //self.mutable();
            });
        }
    }

    fn test3(&mut self) {
        let el = &mut self.elements;
        //self.immutable_helper();
        el.clear();
    }

    fn test4(&mut self) {
        let mut x = 0;

        let el = &self.elements;
        self.immutable_helper();
        //self.mutable();
        profile_fn_mut(|| {
            //self.mutable();
            self.immutable_helper();
            x += 1;
        });
        let l = el.len();
    }

    fn ready(&mut self) {

        let mut x = 1;

        /*
        let y = profile3(|| {
            self.state = None;
            x += 1;
            self.immutable_helper();
            self.mutable();
            self.immutable_helper()
        });
        */

        /*
        for _x in &self.elements {
            profile_fn_mut(|| {
                self.immutable_helper();
                self.mutable();
                x += self.elements[0];
            });
        }
        */

        self.immutable_helper();
    }

    fn mutable(&mut self) {
        self.elements.clear();
    }

    fn immutable_helper(&self) -> i32 {
        42
    }
}
