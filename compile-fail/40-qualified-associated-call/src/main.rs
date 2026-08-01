trait Format {
    fn label() -> &'static str;
}

struct Compact;

impl Format for Compact {
    fn label() -> &'static str {
        "compact"
    }
}

fn main() {
    println!("{}", Format::label());
}
