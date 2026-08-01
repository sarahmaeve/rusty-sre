use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(Vec::new()));
    let task_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut values = task_state.lock().unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        values.push(1);
    })
    .await
    .unwrap();
}
