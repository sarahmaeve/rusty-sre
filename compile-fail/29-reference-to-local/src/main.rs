fn service_label() -> &str {
    let label = String::from("checkout-primary");
    label.as_str()
}

fn main() {
    println!("{}", service_label());
}
