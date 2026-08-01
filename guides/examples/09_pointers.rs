//! Smart pointers add ownership or mutation policies. `Box` owns one heap value,
//! `Rc` and `Arc` share ownership, and cells permit controlled interior mutation.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
//! - <https://doc.rust-lang.org/std/cell/index.html>
//! - <https://doc.rust-lang.org/std/rc/struct.Rc.html>
//! - <https://doc.rust-lang.org/std/sync/struct.Arc.html>
//! - Source study: <https://github.com/servo/servo/tree/main/components/script/dom>

use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
    sync::Arc,
};

#[derive(Debug, PartialEq, Eq)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
struct Service {
    name: String,
    checks: RefCell<Vec<String>>,
    runs: Cell<u64>,
    parent: RefCell<Weak<Service>>,
}

impl Service {
    fn record(&self, check: impl Into<String>) {
        self.runs.set(self.runs.get() + 1);
        self.checks.borrow_mut().push(check.into());
    }
}

struct Guard<'a> {
    events: &'a RefCell<Vec<&'static str>>,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.events.borrow_mut().push("released");
    }
}

fn arc_identity(value: Arc<String>) -> Arc<String> {
    value
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    assert_eq!(
        list,
        List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
    );

    let parent = Rc::new(Service {
        name: "api".to_owned(),
        checks: RefCell::new(Vec::new()),
        runs: Cell::new(0),
        parent: RefCell::new(Weak::new()),
    });
    let child = Rc::new(Service {
        name: "worker".to_owned(),
        checks: RefCell::new(Vec::new()),
        runs: Cell::new(0),
        parent: RefCell::new(Rc::downgrade(&parent)),
    });

    parent.record("ready");
    parent.record("live");
    assert_eq!(parent.runs.get(), 2);
    assert_eq!(&*parent.checks.borrow(), &["ready", "live"]);
    assert_eq!(child.parent.borrow().upgrade().unwrap().name, "api");
    assert_eq!(Rc::strong_count(&parent), 1);
    assert_eq!(Rc::weak_count(&parent), 1);

    // `Arc` provides atomic reference counts, not automatic thread safety for
    // its contents. `Arc<T>` is shareable across threads only when `T` is.
    let shared = Arc::new(String::from("configuration"));
    let returned = arc_identity(Arc::clone(&shared));
    assert!(Arc::ptr_eq(&shared, &returned));
    assert_eq!(Arc::strong_count(&shared), 2);
    drop(returned);
    assert_eq!(Arc::strong_count(&shared), 1);

    let events = RefCell::new(Vec::new());
    {
        let _guard = Guard { events: &events };
        events.borrow_mut().push("acquired");
    } // `Drop::drop` runs on normal scope exit and during unwinding.
    assert_eq!(&*events.borrow(), &["acquired", "released"]);
}
