# Candidate source and change context

```rust
while let Some(batch) = receiver.recv().await {
    let mut state = shared.lock().await;
    state.mark_sending(batch.id());
    send_with_retry(&client, batch).await?;
    state.mark_sent();
}
```

```rust
async fn send_with_retry(client: &Client, batch: Batch) -> Result<(), SendError> {
    for attempt in 0..MAX_ATTEMPTS {
        match client.send(batch.clone()).await {
            Ok(()) => return Ok(()),
            Err(error) if error.is_retryable() => {
                tokio::time::sleep(backoff(attempt)).await;
            }
            Err(error) => return Err(error),
        }
    }
    Err(SendError::Exhausted)
}
```

```rust
async fn shutdown(self) {
    self.cancel.cancel();
    drop(self.sender);
    let _ = self.worker.await;
}
```

Review notes from the batching change mention that metrics reads share `state`.
The HTTP client has a connect timeout. The worker token is cloned at construction,
but no cancellation branch is visible in the selected loop.

Decide which observations establish the incident's immediate cause and which code
still needs inspection before choosing a repair. Avoid treating retry backoff as
causal when the first attempt never finishes.
