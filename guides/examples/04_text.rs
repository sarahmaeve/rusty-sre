//! Rust strings are UTF-8. `String` owns mutable text; `str` is a borrowed text
//! slice. Paths are platform-native values and are not necessarily Unicode.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch08-02-strings.html>
//! - <https://doc.rust-lang.org/std/primitive.str.html>
//! - <https://doc.rust-lang.org/std/path/index.html>
//! - <https://doc.rust-lang.org/std/ffi/struct.OsStr.html>
//! - Source study: <https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore>

use std::path::{Path, PathBuf};

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or("")
}

fn extension(path: &Path) -> Option<&str> {
    // `to_str` is fallible because a platform path need not be valid UTF-8.
    path.extension()?.to_str()
}

fn main() {
    let mut message = String::from("disk");
    message.push(' ');
    message.push_str("full");
    assert_eq!(first_line(&message), "disk full");

    let text = "aé🦀";
    assert_eq!(text.len(), 7); // Bytes, not characters.
    assert_eq!(text.chars().count(), 3);
    assert_eq!(text.as_bytes(), &[97, 195, 169, 240, 159, 166, 128]);

    let boundaries: Vec<_> = text.char_indices().collect();
    assert_eq!(boundaries, vec![(0, 'a'), (1, 'é'), (3, '🦀')]);
    assert_eq!(&text[1..3], "é");
    // `&text[1..2]` would panic because byte 2 is inside `é`.

    let service = "paiements-東京";
    assert_eq!(service.chars().take(10).collect::<String>(), "paiements-");
    assert_eq!(service.get(..10), Some("paiements-"));

    let mut path = PathBuf::from("var");
    path.push("log");
    path.push("agent.json");
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("agent.json")
    );
    assert_eq!(extension(&path), Some("json"));

    let raw: &[u8] = b"HTTP/1.1";
    let protocol = std::str::from_utf8(raw).unwrap();
    assert_eq!(protocol, "HTTP/1.1");
}
