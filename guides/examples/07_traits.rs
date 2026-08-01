//! Traits describe shared behavior. Generics use static dispatch; trait objects
//! use dynamic dispatch. Associated types name one type chosen by an implementor.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch10-02-traits.html>
//! - <https://doc.rust-lang.org/book/ch18-02-trait-objects.html>
//! - <https://doc.rust-lang.org/reference/items/associated-items.html#associated-types>
//! - <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
//! - Source study: <https://github.com/tower-rs/tower/blob/master/tower-service/src/lib.rs>

#[derive(Debug, Clone, PartialEq, Eq)]
struct Alert {
    message: String,
}

trait Source {
    type Error;

    fn next_alert(&mut self) -> Result<Option<Alert>, Self::Error>;
}

struct MemorySource {
    alerts: Vec<Alert>,
}

impl Source for MemorySource {
    type Error = std::convert::Infallible;

    fn next_alert(&mut self) -> Result<Option<Alert>, Self::Error> {
        Ok(self.alerts.pop())
    }
}

fn drain<S>(source: &mut S) -> Result<Vec<Alert>, S::Error>
where
    S: Source,
{
    let mut alerts = Vec::new();
    while let Some(alert) = source.next_alert()? {
        alerts.push(alert);
    }
    Ok(alerts)
}

trait Render {
    fn render(&self) -> String;
}

struct Plain(Alert);
struct Json(Alert);

impl Render for Plain {
    fn render(&self) -> String {
        self.0.message.clone()
    }
}

impl Render for Json {
    fn render(&self) -> String {
        serde_json::json!({ "message": self.0.message.as_str() }).to_string()
    }
}

fn render_all(renderers: &[Box<dyn Render>]) -> Vec<String> {
    renderers.iter().map(|renderer| renderer.render()).collect()
}

trait Named {
    fn name(&self) -> &str;

    fn label(&self) -> String {
        format!("service={}", self.name())
    }
}

impl Named for Alert {
    fn name(&self) -> &str {
        &self.message
    }
}

fn label(value: &(impl Named + ?Sized)) -> String {
    value.label()
}

fn main() {
    let mut source = MemorySource {
        alerts: vec![
            Alert {
                message: "disk".to_owned(),
            },
            Alert {
                message: "latency".to_owned(),
            },
        ],
    };
    let alerts = drain(&mut source).unwrap();
    assert_eq!(alerts[0].message, "latency");

    let renderers: Vec<Box<dyn Render>> = vec![
        Box::new(Plain(alerts[0].clone())),
        Box::new(Json(alerts[1].clone())),
    ];
    assert_eq!(render_all(&renderers), ["latency", r#"{"message":"disk"}"#]);
    let escaped = Json(Alert {
        message: "quote: \"; newline:\n".to_owned(),
    })
    .render();
    let decoded: serde_json::Value = serde_json::from_str(&escaped).unwrap();
    assert_eq!(decoded["message"], "quote: \"; newline:\n");
    assert_eq!(label(&alerts[0]), "service=latency");
}
