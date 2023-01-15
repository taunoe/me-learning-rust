fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // cast
    let average = (a as f64 + b + c as f64) / 3.0;

    println!("average = {}", average);  // 45.1

    if average == 45.1 {
        println!("Test passed!");
    } else {
        println!("Wrong ansver!");
    }
}
