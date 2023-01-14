
// Statement
fn say_hi() {
    let tere = "Tere sõber!";
    println!("{tere}")
}

// Expression
fn kiisu_hi() -> String {
    let tere = "Tere kiisu!".to_string();
    tere
}

fn mitu_rida() {
    println!("{}", "Tere
vana
kere.")
}

fn yks_rida() {
    println!("{}", "Tere \
    vana \
    kere.")
}


fn main() {
    println!("Hi, mumm!");
    say_hi();
    println!("{}", kiisu_hi());

    let (x, y) = (5, 10);
    println!("{} ja {}", x, y);
    println!("{}", x+y);

    //
    macro_rules! make_it {
        ( $var:ident => $($count:expr),+) => {
          $($var.push($count);)+
        }
    }
    
    let mut count = vec![];
    
    make_it![count => u8::MIN, 1, 2];
    
    println!("{count:?}");

    //
    let number = {
        let yks = 1;
        let kaks = 2;
        yks + kaks
    };

    println!("{number}");

    //
    let stack_1 = 32;
    let stack_2 = stack_1; // The value of `stack_1` is copied into `stack_2`
 
    // We now have two values we can work with
    println!("{stack_1}");
    println!("{stack_2}");

    //
    mitu_rida();
    yks_rida();

    let mut number = 12;
    print!("{}\n", number);
    number = 13;
    println!("{}", number);

    //
    //let  a = 20; // Waring
    let _b = 21; // No warnings
    let _ = 22;

    let tõde = true;
    let õigus = false;

    if tõde == õigus {
        println!("{}", "Tõde on õigus");
    } else {
        println!("{}", "Tõde ei ole õigus");
    }

    let tekst = "Kas maakera on lapik?";
    println!("Tähti {}", str::len(tekst));
    println!("Tähti {}", tekst.len());

}
