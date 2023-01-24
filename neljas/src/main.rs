fn main() {
    say_hi();
    say_number(12);
    let x = 2;
    let y = 3;
    liida(x, y);
    let z = 100;
}

fn say_hi() {
    println!("Hi, mom!");
}

fn say_number(number: i32) {
    println!("Number is {}.", number);
}

fn liida(a: u8, b: u8) {
    let sum = a+b;
    println!("Summa on {}.", sum);
}

/*
fn say_name(name: str) {
    println!("Name is {}", name);
}
*/