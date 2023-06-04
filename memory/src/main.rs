struct Elukas {
    ees_nimi: String,
    pere_name: String,
    vanus: i32,
    pikkus: i32,
    sugu: String,
    loom: String,
}

fn main() {
    // This data is on stack
    let tauno = Elukas {
        // String struct is also on stack,
        // but holds a reference to data on heap
        ees_nimi: String::from("Tauno"),
        pere_name: String::from("Erik"),
        vanus: 39,
        pikkus: 1830,
        sugu: String::from("isane"),
        loom: String::from("Inimene"),
    };

    let pässu = Elukas {
        ees_nimi: String::from("Pässu"),
        pere_name: String::from("Karvane"),
        vanus: 8,
        pikkus: 450,
        sugu:  String::from("isane"),
        loom: String::from("Kass"),
    };

    println!(
        "{} on {}, kes on {} aastat vana {} elukas.",
        tauno.ees_nimi, tauno.loom, tauno.vanus, tauno.sugu);
    
    println!(
        "{} on {}, kes on {} aastat vana {} elukas.",
        pässu.ees_nimi, pässu.loom, pässu.vanus, pässu.sugu);

}
