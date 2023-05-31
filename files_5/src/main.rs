// Fuksioonid

fn hello(name: String) {
    println!("Hello {}!", name);
}

fn numbrid(x: i32, y: i32, z: u32) {
    println!("{}, {} ja {}.", x, y, z);
}

fn main() {
    hello("Liv".to_string());

    numbrid(11, -22, 33);
}
