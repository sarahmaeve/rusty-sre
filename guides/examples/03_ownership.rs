//! Ownership determines who may use and eventually drop a value. Borrowing
//! grants temporary access without transferring ownership.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>
//! - <https://doc.rust-lang.org/std/marker/trait.Copy.html>
//! - <https://doc.rust-lang.org/std/clone/trait.Clone.html>
//! - <https://doc.rust-lang.org/nomicon/ownership.html>
//! - Source study: <https://github.com/BurntSushi/ripgrep/tree/master/crates/core>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port(u16);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Endpoint {
    host: String,
    port: Port,
}

fn display(endpoint: &Endpoint) -> String {
    format!("{}:{}", endpoint.host, endpoint.port.0)
}

fn rename(endpoint: &mut Endpoint, host: impl Into<String>) {
    endpoint.host = host.into();
}

fn take(endpoint: Endpoint) -> String {
    endpoint.host
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn main() {
    let port = Port(443);
    let copied = port; // `Port: Copy`, so both bindings remain usable.
    assert_eq!(port, copied);

    let mut endpoint = Endpoint {
        host: "api.internal".to_owned(),
        port,
    };

    let label = display(&endpoint); // Shared borrow.
    assert_eq!(label, "api.internal:443");
    rename(&mut endpoint, "api.service"); // Exclusive borrow.

    // A borrow normally ends after its last use, not at the closing brace.
    let host = &endpoint.host;
    assert_eq!(host, "api.service");
    endpoint.port = Port(8443);

    let replica = endpoint.clone(); // Explicit deep copy of the `String`.
    assert_eq!(replica, endpoint);
    let owned_host = take(replica); // Move; `replica` cannot be used again.
    assert_eq!(owned_host, "api.service");
    assert_eq!(display(&endpoint), "api.service:8443");

    let primary = String::from("primary");
    let secondary = String::from("secondary-node");
    let selected = longest(&primary, &secondary);
    assert_eq!(selected, "secondary-node");

    // Moving one field does not necessarily move the other fields.
    let Endpoint { host, port } = endpoint;
    assert_eq!(host, "api.service");
    assert_eq!(port, Port(8443));
}
