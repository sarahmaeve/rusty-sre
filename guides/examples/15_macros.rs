//! Declarative macros match token patterns and emit syntax. Procedural macros
//! run during compilation and may generate APIs not visible at the call site.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch20-05-macros.html>
//! - <https://doc.rust-lang.org/reference/macros-by-example.html>
//! - <https://doc.rust-lang.org/reference/procedural-macros.html>
//! - <https://rustc-dev-guide.rust-lang.org/macro-expansion.html>
//! - Source study: <https://github.com/tokio-rs/tokio/tree/master/tokio-macros/src>

use std::collections::BTreeMap;

macro_rules! metrics {
    ($( $name:literal => $value:expr ),+ $(,)?) => {{
        let mut result = BTreeMap::new();
        $(
            result.insert($name, $value);
        )+
        result
    }};
}

macro_rules! twice {
    ($value:expr) => {{
        // The temporary is hygienic: it does not collide with `value` at the
        // invocation site. Evaluating the expression once also preserves effects.
        let value = $value;
        value + value
    }};
}

macro_rules! ensure {
    ($condition:expr, $error:expr $(,)?) => {
        if !$condition {
            return Err($error);
        }
    };
}

fn normalize(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

macro_rules! normalized {
    ($input:expr) => {
        // `$crate` resolves to the defining crate. Exported macros use it so a
        // caller's dependency rename does not change internal paths.
        $crate::normalize($input)
    };
}

fn parse_percent(input: &str) -> Result<u8, &'static str> {
    let Ok(value) = input.parse() else {
        return Err("not a number");
    };
    ensure!(value <= 100, "out of range");
    Ok(value)
}

#[tokio::main(flavor = "current_thread")]
async fn generated_runtime() -> usize {
    // The attribute procedural macro rewrites this item into a synchronous
    // function that builds a runtime and drives the async body.
    tokio::task::yield_now().await;
    42
}

fn main() {
    let values = metrics! {
        "requests" => 12_u64,
        "errors" => 2_u64,
    };
    assert_eq!(values, BTreeMap::from([("errors", 2), ("requests", 12)]));

    let value = 100;
    assert_eq!(twice!(3 + 1), 8);
    assert_eq!(value, 100);
    assert_eq!(normalized!(" API "), "api");
    assert_eq!(parse_percent("97"), Ok(97));
    assert_eq!(parse_percent("101"), Err("out of range"));
    assert_eq!(generated_runtime(), 42);

    // When generated methods or trait impls are surprising, inspect the macro's
    // documented expansion contract and source. Expansion tools are diagnostic;
    // the stable API contract still belongs in documentation and tests.
}
