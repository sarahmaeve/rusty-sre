use std::sync::Mutex;

static MODE: Mutex<&'static str> = Mutex::new("normal");

pub fn require_positive(raw: &str) -> u32 {
    let value = raw.parse::<u32>().expect("value must be an integer");
    let _scaled = 100 / value;
    assert!(value > 0, "value must be positive");
    value
}

pub fn current_mode() -> &'static str {
    *MODE.lock().unwrap()
}

pub fn with_mode<T>(mode: &'static str, operation: impl FnOnce() -> T) -> T {
    let previous = {
        let mut current = MODE.lock().unwrap();
        std::mem::replace(&mut *current, mode)
    };
    let result = operation();
    *MODE.lock().unwrap() = previous;
    result
}
