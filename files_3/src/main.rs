/*
Leia read, kus on otsitav sõna.
 */
use std::fs;

fn main() {
    let wanted_string = "kass";
    let file_path = "test_file.txt";

    let content: String = fs::read_to_string(file_path).expect("Unable to open file!");

    for line in content.lines() {
        if line.contains(wanted_string) {
            println!("{}", line);
        }
    }
}
