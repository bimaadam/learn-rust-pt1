pub fn run() {
    // immutable
    let angka = 10;
    println!("Angka: {}", angka);

    // mutable
    let mut total = 5;
    total += 5;
    println!("Total: {}", total);

    // string slice
    let nama: &str = "Bima";
    println!("Nama: {}", nama);

    // build a owned String idiomatically
    let full_name = format!("{} {}", "Bima", "Adam");
    println!("Full name: {}", full_name);

    // shadowing
    let x = 2;
    let x = x * 10;
    println!("Shadowing x: {}", x);
}