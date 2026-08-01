use tokio::sync::mpsc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Envelope {
    pub sequence: u64,
    pub payload: String,
}

pub async fn ingest_batch(events: Vec<Envelope>, channel_capacity: usize) -> Vec<Envelope> {
    let (sender, mut receiver) = mpsc::channel(channel_capacity.max(1));
    let producer = tokio::spawn(async move {
        for event in events {
            let _ = sender.send(event).await;
        }
    });

    let mut stored = Vec::new();
    if let Some(event) = receiver.recv().await {
        stored.push(event);
    }
    drop(receiver);
    let _ = producer.await;
    stored
}
