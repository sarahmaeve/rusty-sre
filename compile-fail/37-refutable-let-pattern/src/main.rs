enum Event {
    Sample(u64),
    Shutdown,
}

fn next_event(shutdown: bool) -> Event {
    if shutdown {
        Event::Shutdown
    } else {
        Event::Sample(42)
    }
}

fn main() {
    let Event::Sample(value) = next_event(false);
    println!("sample={value}");
}
