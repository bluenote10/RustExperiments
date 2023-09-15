#![allow(dead_code)]

///
/// References:
/// - https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
///
use std::{
    cell::RefCell,
    sync::{Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

struct MyData {
    counter: i32,
}

/// V1 using Mutex

fn singleton_v1() -> &'static Mutex<MyData> {
    static INSTANCE: OnceLock<Mutex<MyData>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(MyData { counter: 0 }))
}

fn get_v1() -> MutexGuard<'static, MyData> {
    singleton_v1().lock().unwrap()
}

fn basic_example_v1() {
    // Interestingly this does not deadlock, i.e., the temporary mutex guards seemed to be dropped
    // immediately.
    get_v1().counter += 1;
    get_v1().counter += 1;
    get_v1().counter += 1;
    // But binding an instance to a local var has a high deadlock potential:
    // let instance = get_v1();
    println!("counter: {}", get_v1().counter);
}

fn deadlock_example_v1() {
    // This simply seems to deadlock due to nested access on same thread

    fn inner_access() {
        get_v1().counter += 1
    }

    let mut outer_access = get_v1();
    inner_access();
    outer_access.counter += 1;
}

/// V2 using RwLock

fn singleton_v2() -> &'static RwLock<MyData> {
    static INSTANCE: OnceLock<RwLock<MyData>> = OnceLock::new();
    INSTANCE.get_or_init(|| RwLock::new(MyData { counter: 0 }))
}

fn get_v2() -> RwLockReadGuard<'static, MyData> {
    singleton_v2().read().unwrap()
}

fn get_mut_v2() -> RwLockWriteGuard<'static, MyData> {
    singleton_v2().write().unwrap()
}

fn basic_example_v2() {
    get_mut_v2().counter += 1;
    get_mut_v2().counter += 1;
    get_mut_v2().counter += 1;
    println!("counter: {}", get_v2().counter);
}

fn deadlock_example_v2() {
    // Note that even though the outer access is only reading, the nested access still
    // deadlocks, because it has to wait for the outer reader to disappear.

    fn inner_access() {
        get_mut_v2().counter += 1
    }

    let _outer_access = get_v2();
    inner_access();
}

/// V3 using RefCell

/*
As expected, the compiler does not allow a static OnceLock<RefCell<...>> because it requires Sync
for static variables, because in general the access can come from arbitrary threads.

fn singleton_v3() -> &'static RefCell<MyData> {
    static ARRAY: OnceLock<RefCell<MyData>> = OnceLock::new();
    ARRAY.get_or_init(|| RefCell::new(MyData { counter: 0 }))
}

error[E0277]: `RefCell<MyData>` cannot be shared between threads safely
  --> src/main.rs:91:19
   |
91 |     static ARRAY: OnceLock<RefCell<MyData>> = OnceLock::new();
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<MyData>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `RefCell<MyData>`
   = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
   = note: required for `OnceLock<RefCell<MyData>>` to implement `Sync`
   = note: shared static variables must have a type that implements `Sync`

This probably means that we cannot have an API the "hands out" any accessor. The `.with` API of thread-local
storage forces to use a callback based access...
*/

fn singleton_v3<F, T>(f: F) -> T
where
    F: FnOnce(&mut MyData) -> T,
{
    thread_local! {
        static INSTANCE: OnceLock<RefCell<MyData>> = OnceLock::new();
    }
    // TODO: This isn't great that we call borrow_mut unconditionally.
    // A real API should probably split this up into two accessors for
    // immutable vs mutable access...
    INSTANCE.with(|instance| {
        f(&mut *instance
            .get_or_init(|| RefCell::new(MyData { counter: 0 }))
            .borrow_mut())
    })
}

fn basic_example_v3() {
    singleton_v3(|data| data.counter += 1);
    singleton_v3(|data| data.counter += 1);
    singleton_v3(|data| data.counter += 1);
    println!("counter: {}", singleton_v3(|data| data.counter));
}

fn deadlock_example_v3() {
    // This detects the deadlock via a BorrowMutError panic.
    fn inner_access() {
        singleton_v3(|data| data.counter += 1);
    }

    singleton_v3(|data| {
        data.counter += 1;
        inner_access();
    });
}

fn main() {
    deadlock_example_v3();
}
