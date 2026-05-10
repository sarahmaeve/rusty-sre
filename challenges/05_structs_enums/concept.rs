// =============================================================================
// Challenge 05: Structs, Enums, and impl — Concept Explainer
// =============================================================================
//
// Earlier challenges defined structs and enums but never explained the
// type-design mechanics. This challenge covers:
//
//   - struct definition and instantiation
//   - methods (&self, &mut self, self) and associated functions
//   - enums with payload data (Rust's "sum types")
//   - match on enums (exhaustive by default)
//   - the impl block — separate from the type definition
//
// No traits, no generics, no derive yet — those are 10 and beyond.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================
//
// Python → Rust quick reference:
//
//   class Foo:                       struct Foo { x: i32 }
//                                    impl Foo { ... }
//   def __init__(self, x):           fn new(x: i32) -> Self { Self { x } }
//   def method(self):                fn method(&self) { ... }       (read-only)
//   def method(self):                fn method(&mut self) { ... }   (mutating)
//   def method(self):                fn method(self) { ... }        (consumes)
//   class Cat: pass                  struct Cat;                    (unit struct)
//   class Color(Enum):               enum Color { Red, Green, Blue }
//   class Shape:                     enum Shape {
//      def __init__(self, kind):       Circle { radius: f64 },
//        ...                           Rect { w: f64, h: f64 },
//                                    }

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. STRUCTS AND FIELD ACCESS
// =============================================================================

#[test]
fn struct_basics() {
    // A struct with named fields
    struct Service {
        name: String,
        port: u16,
        healthy: bool,
    }

    let svc = Service {
        name: String::from("auth"),
        port: 8080,
        healthy: true,
    };

    assert_eq!(svc.name, "auth");
    assert_eq!(svc.port, 8080);
    assert!(svc.healthy);

    // Struct update syntax: copy fields from another instance, override some.
    // The `..other` part must come last and provides the remaining fields.
    let svc2 = Service {
        port: 9090,
        ..svc
    };
    assert_eq!(svc2.name, "auth"); // copied from svc
    assert_eq!(svc2.port, 9090); // overridden
    // svc.name was MOVED into svc2 because String is not Copy. svc.healthy
    // is still readable because bool IS Copy. (This shows up later when we
    // talk about Clone/Copy in challenge 10.)
    assert!(svc.healthy);
}

// =============================================================================
// 2. METHODS — &self, &mut self, self
// =============================================================================
// Methods live in `impl` blocks separate from the struct definition. The
// receiver controls what the method can do:
//
//   &self      — read-only borrow. The instance is still usable afterward.
//   &mut self  — mutable borrow. The instance is still usable afterward.
//   self       — takes ownership. The instance is consumed.

struct Counter {
    value: i64,
}

impl Counter {
    // Associated function (no `self`) — called as `Counter::new(...)`.
    // Convention: `new` returns Self, the type the impl is for.
    fn new() -> Self {
        Self { value: 0 }
    }

    // Read-only method
    fn current(&self) -> i64 {
        self.value
    }

    // Mutating method
    fn increment(&mut self) {
        self.value += 1;
    }

    // Consuming method — useful when the next state isn't a Counter anymore
    fn finish(self) -> i64 {
        self.value
    }
}

#[test]
fn methods_with_three_receivers() {
    let mut c = Counter::new();
    assert_eq!(c.current(), 0);

    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.current(), 3); // c is still usable

    let final_value = c.finish();
    assert_eq!(final_value, 3);
    // c is no longer usable — finish() consumed it.
    // c.current(); // would fail to compile: "borrow of moved value"
}

// =============================================================================
// 3. ASSOCIATED FUNCTIONS (CONSTRUCTORS AND HELPERS)
// =============================================================================
// Functions inside `impl` that don't take `self` are "associated functions."
// They're called via the type: `Counter::new()`. By convention:
//
//   new(...)            — primary constructor
//   with_value(v: T)    — alternative constructor
//   default()           — implements the Default trait (challenge 10)

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    fn len(&self) -> i32 {
        self.end - self.start
    }
}

#[test]
fn associated_functions() {
    let r = Range::new(10, 25);
    assert_eq!(r.len(), 15);

    let e = Range::empty();
    assert_eq!(e.len(), 0);
}

// =============================================================================
// 4. ENUMS — RUST'S SUM TYPES
// =============================================================================
// An enum lists all the possible "shapes" a value can take. Each variant can
// carry data of its own. This is Python's `Enum` plus algebraic data types.

#[test]
fn enum_basics() {
    enum Severity {
        Info,
        Warn,
        Error,
    }

    let s = Severity::Warn;

    // To consume an enum, use match. Match is exhaustive — the compiler
    // will reject this code if you remove an arm.
    let label = match s {
        Severity::Info => "info",
        Severity::Warn => "warn",
        Severity::Error => "error",
    };
    assert_eq!(label, "warn");
}

// =============================================================================
// 5. ENUMS WITH DATA — DIFFERENT VARIANTS, DIFFERENT FIELDS
// =============================================================================
// Each variant can carry its own payload. This is where enums get powerful.

enum Event {
    Heartbeat,                                    // unit variant — no data
    Login { user: String },                       // struct-like variant
    Error(String),                                // tuple-like variant
    Latency { p50: f64, p99: f64 },              // multi-field struct variant
}

fn describe(e: &Event) -> String {
    match e {
        Event::Heartbeat => "heartbeat".to_string(),
        Event::Login { user } => format!("login by {user}"),
        Event::Error(msg) => format!("error: {msg}"),
        Event::Latency { p50, p99 } => format!("latency p50={p50} p99={p99}"),
    }
}

#[test]
fn enum_with_data() {
    assert_eq!(describe(&Event::Heartbeat), "heartbeat");
    assert_eq!(
        describe(&Event::Login { user: "alice".to_string() }),
        "login by alice"
    );
    assert_eq!(
        describe(&Event::Error("disk full".to_string())),
        "error: disk full"
    );
    assert_eq!(
        describe(&Event::Latency { p50: 50.0, p99: 200.0 }),
        "latency p50=50 p99=200"
    );
}

// =============================================================================
// 6. METHODS ON ENUMS
// =============================================================================
// `impl` works on enums just like on structs.

enum Status {
    Up,
    Down,
    Degraded { reason: String },
}

impl Status {
    fn is_serving(&self) -> bool {
        match self {
            Status::Up => true,
            Status::Degraded { .. } => true, // .. ignores the payload
            Status::Down => false,
        }
    }

    fn label(&self) -> &str {
        match self {
            Status::Up => "up",
            Status::Down => "down",
            Status::Degraded { .. } => "degraded",
        }
    }
}

#[test]
fn enum_methods() {
    assert!(Status::Up.is_serving());
    assert!(!Status::Down.is_serving());

    let d = Status::Degraded { reason: "cache miss".into() };
    assert!(d.is_serving());
    assert_eq!(d.label(), "degraded");
}

// =============================================================================
// 7. PATTERN BINDING — DESTRUCTURING IN MATCH
// =============================================================================
// Match arms can pull values out of a variant for use in the arm body.

#[test]
fn destructuring_in_match() {
    enum Alert {
        Pageable { service: String, severity: u8 },
        Email { to: String },
    }

    let a = Alert::Pageable {
        service: "auth".to_string(),
        severity: 4,
    };

    let summary = match a {
        Alert::Pageable { service, severity } => {
            format!("page {service} (sev={severity})")
        }
        Alert::Email { to } => format!("email {to}"),
    };
    assert_eq!(summary, "page auth (sev=4)");
}

// =============================================================================
// 8. UNIT STRUCTS AND TUPLE STRUCTS
// =============================================================================

#[test]
fn other_struct_shapes() {
    // Unit struct — no fields. Useful for marker types or trait targets.
    struct Marker;
    let _m = Marker;

    // Tuple struct — fields are positional, accessed by .0, .1, ...
    struct Point(f64, f64);
    let p = Point(1.5, 2.5);
    assert_eq!(p.0, 1.5);
    assert_eq!(p.1, 2.5);

    // The "newtype" pattern: a tuple struct around one value, used to give
    // a more specific type to a primitive. (Covered more in challenge 10.)
    struct Port(u16);
    let p = Port(8080);
    assert_eq!(p.0, 8080);
}

// =============================================================================
// 9. WHEN SHOULD I REACH FOR AN ENUM?
// =============================================================================
// Use an enum whenever you find yourself thinking "this can be one of N
// shapes, and they have different data attached." The classic Python-vs-Rust
// difference:
//
//   Python: a function returns a dict, sometimes with `error_code`, sometimes
//           with `result`. Callers have to remember the implicit contract.
//
//   Rust:   the function returns an enum that names every shape, and `match`
//           forces callers to handle all of them. Wrong shape → compile error.

#[test]
fn enum_for_results() {
    // A toy parser result with three outcomes
    enum Parsed {
        Number(i64),
        Text(String),
        Empty,
    }

    fn parse(input: &str) -> Parsed {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Parsed::Empty
        } else if let Ok(n) = trimmed.parse::<i64>() {
            Parsed::Number(n)
        } else {
            Parsed::Text(trimmed.to_string())
        }
    }

    // The match is exhaustive — every variant gets a clear handler.
    let label = match parse("  42 ") {
        Parsed::Number(n) => format!("num={n}"),
        Parsed::Text(s) => format!("txt={s}"),
        Parsed::Empty => "empty".to_string(),
    };
    assert_eq!(label, "num=42");
}
