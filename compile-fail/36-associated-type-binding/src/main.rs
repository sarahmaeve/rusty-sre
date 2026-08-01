trait Source {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct ServiceNames(Vec<String>);

impl Source for ServiceNames {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn drain(mut source: Box<dyn Source>) -> usize {
    let mut count = 0;
    while source.next().is_some() {
        count += 1;
    }
    count
}

fn main() {
    let source = ServiceNames(vec!["api".to_owned(), "db".to_owned()]);
    println!("{}", drain(Box::new(source)));
}
