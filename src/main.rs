use std::fs;
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn main() {
    println!("{}", read_file("test.forth"));
}
