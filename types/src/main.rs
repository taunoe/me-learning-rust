fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    let bv = true;  // boolean
    let x = 12;     // by default this is i32
    let a = 12u8;   // u8
    let b = 4.3;    // by default this is f64
    let c = 4.3f32; // f32
    
    let t = (13, false);  // Tuple

    let ar = [1, 2, 3];  // Array

    let sentence = "hello world!";

    println!(
        "{} {} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence, ar[1]
    );

    /////////////////////////////////////////////
    // Type conversion
    let aa = 13u8;
    let bb = 7u32;
    let cc = aa as u32 + bb;
    println!("{}", cc);

    let t = true;
    println!("{}", t as u8);

    // Constants

    const PI : f32 = 3.1415;

    println!("Pi is {}", PI);

    // Arrays
    let nums: [i32; 4] = [0, 1, 2, 3];
    println!("{:?}", nums);  // Print array
    println!("{}", nums[1]);

    println!("4+2={}", add(4, 2));

    let mut tup = (4, 1);
    println!("{:?}", tup);
    tup = swap(tup.0, tup.1);
    println!("{:?}", tup);
    let(a, b) = swap(tup.0, tup.1);
    println!("{} ja {}", a, b);

    // loop and break
    let mut i = 0;
    loop {
        i += 1;
        println!("{}", i);
        if i == 5 {
            break;
        }
    }

    // for loop
    for x in 0..3 {
        println!("{}", x);
    }

    for x in 0..=3 {
        println!("{}", x);
    }

    // match
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }

    // Returning Values From loop
    let mut x = 0;
    
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);


}
