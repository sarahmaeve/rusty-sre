use std::collections::HashSet;

#[derive(Debug)]
struct Host {
    name: String,
}

fn main() {
    let hosts = HashSet::from([Host {
        name: "api-1".to_owned(),
    }]);
    println!("{}", hosts.len());
}
