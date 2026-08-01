use std::thread;

fn spawn_drop<T: Send>(value: T) -> thread::JoinHandle<()> {
    thread::spawn(move || drop(value))
}

fn main() {
    spawn_drop(String::from("completed batch")).join().unwrap();
}
