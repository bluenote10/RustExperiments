use std::cell::RefCell;
use std::thread;

fn fake_static() -> i32 {
    thread_local!(static COUNTER : i32 = 0);
    COUNTER.with(|c| {
        println!("{}", c);
        *c
    })
}

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

fn main() {
    println!("{}", fake_static());

    let t1 = thread::spawn(move || {
        println!("{}", fake_static());
    });
    t1.join().unwrap();

    // Example from:
    // https://doc.rust-lang.org/std/thread/struct.LocalKey.html#examples

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // each thread starts out with the initial value of 1, even though this thread already changed its copy of the thread local value to 2
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // original thread retains the original value of 2 despite the child thread changing the value to 3 for that thread
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    // wait for the thread to complete and bail out on panic
    t.join().unwrap();

    // each thread starts out with the initial value of 1, even though this thread already changed its copy of the thread local value to 2
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 4;
        });
    });

    // original thread retains the original value of 2 despite the child thread changing the value to 3 for that thread
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    // wait for the thread to complete and bail out on panic
    t.join().unwrap();

    // original thread retains the original value of 2 despite the child thread changing the value to 3 for that thread
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}
