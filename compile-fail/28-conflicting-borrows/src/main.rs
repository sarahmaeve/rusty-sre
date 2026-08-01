fn main() {
    let mut hosts = vec!["api-1".to_owned(), "db-1".to_owned()];
    let api = hosts.iter().find(|host| host.starts_with("api"));
    hosts.push("api-2".to_owned());
    println!("first API host: {api:?}");
}
