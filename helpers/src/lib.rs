use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
