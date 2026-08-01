use std::marker::PhantomPinned;
use std::pin::Pin;

struct Worker {
    name: String,
    _pin: PhantomPinned,
}

fn main() {
    let worker = Box::pin(Worker {
        name: "collector".to_owned(),
        _pin: PhantomPinned,
    });
    let worker = Pin::into_inner(worker);
    println!("{}", worker.name);
}
