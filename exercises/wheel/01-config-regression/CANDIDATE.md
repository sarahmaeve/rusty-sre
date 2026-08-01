# Candidate source and change context

The release changed these nearby paths. The excerpts are abbreviated; line order is
preserved.

```rust
let raw_limit = env.get("WORKER_LIMIT");
let worker_limit = raw_limit
    .and_then(|value| value.parse::<usize>().ok())
    .unwrap_or(DEFAULT_WORKERS);

let source = if raw_limit.is_some() {
    ConfigSource::Environment
} else {
    ConfigSource::Default
};
```

```rust
#[derive(Serialize, Deserialize)]
struct Snapshot {
    worker_limit: usize,
    #[serde(default, skip_serializing_if = "is_false")]
    snapshot_enabled: bool,
}
```

```rust
fn is_false(value: &bool) -> bool {
    !*value
}
```

Change summary:

- snapshot export moved after worker-pool construction;
- startup logging gained a source label;
- the parser was rewritten from a match to an iterator-style chain;
- the default worker count was unchanged.

Classify each suspicious item as causal, contributory, unrelated, or still
unresolved. Do not assume every new line belongs in the repair.
