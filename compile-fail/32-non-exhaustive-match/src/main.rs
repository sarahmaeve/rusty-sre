enum State {
    Healthy,
    Degraded,
    Draining,
}

fn is_available(state: State) -> bool {
    match state {
        State::Healthy => true,
        State::Degraded => false,
    }
}

fn main() {
    println!("{}", is_available(State::Healthy));
    let _ = (State::Degraded, State::Draining);
}
