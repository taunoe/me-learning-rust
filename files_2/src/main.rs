use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let file_path = "test_file.txt";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
