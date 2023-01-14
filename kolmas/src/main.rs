fn main() {
    println!("Hello, world!");

    let a = 10;
    let mut b = 13;
    println!("a = {} & b = {}", a, b);

    b += 1;
    println!("a = {} & b = {}", a, b);

    let c: u8 = 255;
    println!("c = {}", c);

    let d = 10.1234567890123456789; // f64, 15 kohta peale koma
    println!("d = {}", d);

    let e: f32 = 10.123456789; // f32, 6 kohta peale koma
    println!("d = {}", e);
}
