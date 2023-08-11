#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::sync::MutexGuard;
use std::thread;

fn is_sync<T: Sync>(_x: T) {}
fn is_send<T: Send>(_x: T) {}

struct NotSendNotSync {
    _marker: PhantomData<*const ()>,
}

impl NotSendNotSync {
    fn share(&self) -> NotSendNotSync {
        NotSendNotSync {
            _marker: PhantomData,
        }
    }
}

struct NotSync {
    _marker: PhantomData<Cell<()>>,
}

struct NotSend {
    _marker: PhantomData<MutexGuard<'static, ()>>,
}

fn check() {
    // is_send(NotSendNotSync {
    //     _marker: PhantomData,
    // });
    // is_sync(NotSendNotSync {
    //     _marker: PhantomData,
    // });

    is_send(NotSync {
        _marker: PhantomData,
    });
    // is_sync(NotSync {
    //     _marker: PhantomData,
    // });

    // is_send(NotSend {
    //     _marker: PhantomData,
    // });
    is_sync(NotSend {
        _marker: PhantomData,
    });
}

fn fake_static_not_sync_not_send() -> NotSendNotSync {
    thread_local!(static INSTANCE : NotSendNotSync = NotSendNotSync{ _marker: PhantomData});
    INSTANCE.with(|instance| instance.share())
}

fn fake_static_not_sync_not_send_lazy() -> NotSendNotSync {
    // Following the pattern: https://stackoverflow.com/a/76879555/1804173
    use once_cell::unsync::Lazy;

    thread_local! {
        static INSTANCE: Lazy<NotSendNotSync> = Lazy::new(||
            NotSendNotSync{ _marker: PhantomData}
        );
    }

    INSTANCE.with(|instance| instance.share())
}

fn fake_static_i32() -> i32 {
    thread_local!(static COUNTER : i32 = 0);
    COUNTER.with(|c| {
        println!("{}", c);
        *c
    })
}

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

fn main() {
    println!("{}", fake_static_i32());

    let t1 = thread::spawn(move || {
        println!("{}", fake_static_i32());
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
