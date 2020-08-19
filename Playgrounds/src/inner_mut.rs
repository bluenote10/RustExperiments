use std::cell::RefCell;
use std::sync::Arc;


struct Data {
    data: i32
}


fn test() {
    let x = Arc::new(RefCell::new(Data{data: 0}));
}
