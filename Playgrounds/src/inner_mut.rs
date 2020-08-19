use std::cell::RefCell;
use std::sync::Arc;


#[derive(Debug)]
struct Data {
    data: i32
}


pub fn test() {
    let x = Arc::new(RefCell::new(Data{data: 0}));

    {
        let x_mut = x.borrow();
        println!("{:?}", x_mut);
    }
    {
        let mut x_mut = x.borrow_mut();
        x_mut.data = 1;
    }
    {
        let x_mut = x.borrow();
        println!("{:?}", x_mut);
    }
}
