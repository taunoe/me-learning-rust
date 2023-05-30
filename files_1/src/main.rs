use std::fs;

fn main() {
    let file_path = "test_file.txt";
    // Read file content
    let contents: String = fs::read_to_string(file_path).expect("Unable to read file");
    // Print all
    println!("{}", contents);
}
