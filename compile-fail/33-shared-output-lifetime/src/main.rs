fn choose_label<'a>(primary: &'a str, fallback: &str, primary_ready: bool) -> &'a str {
    if primary_ready { primary } else { fallback }
}

fn main() {
    let primary = String::from("api-primary");
    let fallback = String::from("api-fallback");
    println!("{}", choose_label(&primary, &fallback, false));
}
