fn summarize(lines: Vec<String>) -> usize {
    lines.iter().filter(|line| line.contains("ERROR")).count()
}

fn main() {
    let lines = vec!["INFO ready".to_owned(), "ERROR timeout".to_owned()];
    let errors = summarize(lines);
    println!("{errors} errors in {} lines", lines.len());
}
