fn main() {
    say_hi();
    say_number(12);
    let x = 2;
    let y = 3;
    liida(x, y);

    let z = 4;
    squares(z);
    squares_2(z);
    let result = squares_3(z);
    println!("result is {:?}", result);

    let cf = 23.0;
    let tf = celcius_to_fahrenheit(cf);
    println!("{}C is {}F", cf, tf);

    assert_eq!(tf, 73.4);
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

fn squares(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x  // Return
}

fn squares_2(x: i32) -> i32 {
    println!("squaring_2 {}", x);
    return x * x;
}

fn squares_3(x: i32) -> (i32, i32) {
    println!("squaring_2 {}", x);
    return (x, x * x);
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    let f = (1.8 * c ) + 32.0;
    return f;
}