#![allow(dead_code)]

use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, MutexGuard};
use std::thread::{self, sleep};
use std::time::Duration;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

type SendAndSync = i32;
type SendNotSync = Cell<i32>;
type SyncNotSend = MutexGuard<'static, i32>;
type NeitherSendNorSync = Rc<i32>;

assert_impl_all!(i32: Send, Sync);
assert_impl_all!(&i32: Send, Sync);
assert_impl_all!(&mut i32: Send, Sync);

assert_impl_all!(String: Send, Sync);
assert_impl_all!(&str: Send, Sync);
assert_impl_all!(&mut str: Send, Sync);

assert_impl_all!(Box<i32>: Sync, Send);

// Rc is neither Sync nor Send.
assert_not_impl_any!(Rc<i32>: Sync, Send);

// Arc is both (if the underlying T is both Send and Sync)
assert_impl_all!(Arc<SendAndSync>: Send, Sync);
assert_not_impl_any!(Arc<SendNotSync>: Send, Sync);
assert_not_impl_any!(Arc<SyncNotSend>: Send, Sync);
assert_not_impl_any!(Arc<NeitherSendNorSync>: Send, Sync);

// Cell types are Send, but not Sync.
assert_impl_all!(Cell<i32>: Send);
assert_not_impl_any!(Cell<i32>: Sync);
assert_impl_all!(RefCell<i32>: Send);
assert_not_impl_any!(RefCell<i32>: Sync);

// Mutex guard is an example of a type that is !Send but Sync.
// https://users.rust-lang.org/t/example-of-a-type-that-is-not-send/59835/3
assert_not_impl_any!(MutexGuard<i32>: Send);
assert_impl_all!(MutexGuard<i32>: Sync);

// If a struct is composed of types that don't have interior mutability, it is
// Send + Sync. This makes sense, because without interior mutability, the compiler
// knows that sharing &T is fine -- it is guaranteed immutable.
// Of course `Sync` refers only to &T, not &mut T -- sending a &mut T to a thread
// is of course prohibited, because &mut is by definition "not shareable".
struct S1 {
    a: i32,
    b: f32,
    c: String,
    d: Vec<i32>,
}
assert_impl_all!(S1: Send, Sync);

// As soon as a struct incorporates types with interior mutability, the Send/Sync
// properties of this underlying type get "forwarded".
struct S2 {
    a: i32,
    b: f32,
    c: String,
    d: Vec<RefCell<i32>>,
}
assert_impl_all!(S2: Send);
assert_not_impl_any!(S2: Sync);

pub fn is_send<T: Send>(_x: T) {}
pub fn is_sync<T: Sync>(_x: T) {}

pub fn manual_checking() {
    is_send(0_i32);
    is_sync(0_i32);
    is_send(&0_i32);
    is_sync(&0_i32);
}

pub fn example_i32() {
    let x = 0_i32;

    // We have to use move closures to opt-out of the compilers "minimal capture behavior",
    // which would try to capture x by reference -- but one that doesn't live long enough.
    // Using a move closure practically means to invoke `Copy` on `x`, i.e., there is no
    // sharing at all.
    thread::spawn(move || {
        println!("thread 1: {}", x);
    });
    thread::spawn(move || {
        println!("thread 1: {}", x);
    });
}

/*
pub fn example_i32_ref() {
    // For a local variable, passing &i32 doesn't work, because the lifetime is insufficient.
    let x = 0_i32;
    let x_ref = &x;
    thread::spawn(move || {
        println!("thread 1: {}", x_ref);
    });
    thread::spawn(move || {
        println!("thread 1: {}", x_ref);
    });
}
*/

static SOME_I32: i32 = 0;

pub fn example_i32_static() {
    // Passing an &i32 *is* possible if its lifetime is long enough, which means it must
    // have 'static lifetime.
    let x = &SOME_I32;

    thread::spawn(move || {
        println!("thread 1: {}", x);
        // *x += 1; Of course, we are behind a &T reference and lack interior mutability.
    });
    thread::spawn(move || {
        println!("thread 1: {}", x);
        // *x += 1; Of course, we are behind a &T reference and lack interior mutability.
    });
}

/*
pub fn example_i32_static_mut() {
    // Passing a &'static mut T to a thread is obviously also disallowed.
    let x = &mut SOME_I32;

    thread::spawn(move || {
        println!("thread 1: {}", x);
    });
    thread::spawn(move || {
        println!("thread 1: {}", x);
    });
}
*/

pub fn example_arc_i32() {
    // Wrapping a local variable into Arc allows to pass it into the threads via the
    // (atomic) ref counted smart pointer.
    // However, Arc itself doesn't provide interior mutability. Therefore everything is
    // immutable, and no communication can happen.
    let x = Arc::new(0_i32);

    {
        let x = x.clone();
        thread::spawn(move || {
            println!("{}", x);
            // *x += 1; Of course, Arc doesn't provide DerefMut, so no mutation possible
        });
    }
    {
        let x = x.clone();
        thread::spawn(move || {
            println!("{}", x);
            // *x += 1; Of course, Arc doesn't provide DerefMut, so no mutation possible
        });
    }

    for _ in 0..10 {
        // *x += 1; Of course, Arc doesn't provide DerefMut, so no mutation possible
    }
}

pub fn example_arc_atomic_i32() {
    // The combination of Arc<AtomicXXX<...>> does the trick:
    // - Arc solves the "share / pass to thread" part,
    // - AtomicXXX provides interior mutability to actually modify anything.
    let x = Arc::new(AtomicI32::new(0));

    {
        let x = x.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(30));
            x.fetch_add(1, Ordering::SeqCst);
            println!("[thread 1] {}", x.load(Ordering::SeqCst));
        });
    }
    {
        let x = x.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(60));
            x.fetch_add(1, Ordering::SeqCst);
            println!("[thread 2] {}", x.load(Ordering::SeqCst));
        });
    }

    for _ in 0..10 {
        sleep(Duration::from_millis(10));
        x.fetch_add(1, Ordering::SeqCst);
        println!("[main] {}", x.load(Ordering::SeqCst));
    }
}

static SOME_ATOMIC_I32: AtomicI32 = AtomicI32::new(0);

pub fn example_static_atomic_i32() {
    // Note that it technically *is* possible to use AtomicXXX without Arc wrapping but
    // it comes down to a lifetime question: Instead of sending Arc<AtomicXXX<...>> we
    // can send &AtomicXXX<...> to the thread, but then the reference must live long
    // enough, which again means that it has to have 'static lifetime.
    let x = &SOME_ATOMIC_I32;

    thread::spawn(move || {
        sleep(Duration::from_millis(30));
        x.fetch_add(1, Ordering::SeqCst);
        println!("[thread 1] {}", x.load(Ordering::SeqCst));
    });
    thread::spawn(move || {
        sleep(Duration::from_millis(60));
        x.fetch_add(1, Ordering::SeqCst);
        println!("[thread 2] {}", x.load(Ordering::SeqCst));
    });

    for _ in 0..10 {
        sleep(Duration::from_millis(10));
        x.fetch_add(1, Ordering::SeqCst);
        println!("[main] {}", x.load(Ordering::SeqCst));
    }
}

fn main() {
    example_arc_atomic_i32();
    example_static_atomic_i32();
}
