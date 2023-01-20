use core::num;

fn main() {
    println!("Hello, world!");

    let a = 10; // default integrer is i32
    let mut b = 13;
    println!("a = {} & b = {}", a, b);

    b += 1;
    println!("a = {} & b = {}", a, b);

    let c: u8 = 255;
    println!("c = {}", c);

    // default float is f64
    let d = 10.1234567890123456789; // f64, 15 kohta peale koma
    println!("d = {}", d);

    let e: f32 = 10.123456789; // f32, 6 kohta peale koma
    println!("d = {}", e);

    // Booleans
    let t = true;
    let f = false;
    println!("t is {} and f if {}", t, f);
    println!("NOT t is {}", !t);
    println!("t AND f is {}", t & f);
    println!("t OR f is {}", t | f);
    println!("t XOR f is {}", t ^ f);
    println!("");

    // Chars, Unicode
    // www.unicode.org/charts/PDF/U2600.pdf
    let täht = '♥';  // cahr == 4 byte unicode value
    let number = '9';
    println!("\u{261D} ja {} ja {}", täht, number);

    // array
    let letters = ['a', 'b', 'c'];
    let first_letter = letters[0];
    println!("first letter {}", first_letter);

    let numbers: [i32; 5];
    // init all to 0
    numbers = [0; 5]; //repeat expression
    let index = numbers.len();
    println!("first num {}", numbers[index-1]);

    // multidimentsional arrays
    let multi = [[1, 2, 3],
                                [4, 5, 6]];
    println!("multi [1][1] = {}", multi[1][1]);

    let garage =  [[[0; 100]; 20]; 5];
    println!("{}", garage[1][1][1]);

    // Tuples
    let mut stuff = (10, 3.14, 'x');
    stuff.0 += 1;
    let stuff_1 = stuff.0;
    let stuff_2 = stuff.1;
    println!("stuff: {} {}", stuff_1, stuff_2);

    let (a, b, c) = stuff;
    println!("b is {}", b);


}
