fn main() {
    let n = 9;

    if n > 3 {
        println!("Suurem!");
    }

    println!("{}",
        if n > 7 {
            "big"
        }
        else if n > 0 {
            "small"
        }
        else if n < 0 {
            "negative"
        }
        else {
            "neither positive nor negative"
        }
    );

    let mut i = 1;
    while i <= 10 {
        print!("{} ",i * i);
        i += 1;
    }
    println!("");

    let mut j = 0;
    while j < 50 {
        j += 1;
        if j % 3 == 0 { continue; }
        if j * j > 400 { break; }
        print!("{} ", j * j);
    }
    println!("");

}
