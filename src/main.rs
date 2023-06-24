use std::fs;

fn read_file() -> Vec<String> {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    contents.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let input = read_file();
    let a = 1;
}
