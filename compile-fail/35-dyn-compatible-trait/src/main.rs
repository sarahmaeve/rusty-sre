use std::fmt::Display;

trait Reporter {
    fn name(&self) -> &str;

    fn encode<T: Display>(&self, value: T) -> String {
        format!("{}:{value}", self.name())
    }
}

struct TextReporter;

impl Reporter for TextReporter {
    fn name(&self) -> &str {
        "text"
    }
}

fn reporter_name(reporter: &dyn Reporter) -> &str {
    reporter.name()
}

fn main() {
    let reporter = TextReporter;
    println!("{} {}", reporter_name(&reporter), reporter.encode(7));
}
