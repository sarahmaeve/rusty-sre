macro_rules! is_state {
    ($value:expr, $expected:ident) => {
        matches!($value, $expected)
    };
}

enum State {
    Ready,
    Draining,
}

fn main() {
    let state = if std::env::args().len() > 1 {
        State::Draining
    } else {
        State::Ready
    };
    println!("{}", is_state!(state, State::Ready));
}
