/*
Kirjuta faili. Loob faili kui ei ole. Kirjutab üle.
 */
use std::fs;

fn main() {
    let file_path = "new_file.txt";
    let content = "Tere vana kere!".to_string();
    let data = "Suur on maailma ime!".to_string();

    fs::write(file_path, content).unwrap();
    fs::write(file_path, data).unwrap();  // kirjutab üle faili
}
