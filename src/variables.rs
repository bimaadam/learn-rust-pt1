pub fn run() {
    // immutable
    let angka = 10;
    println!("Angka: {}", angka);

    // mutable
    let mut total = 5;
    total += 5;
    println!("Total: {}", total);

    // string
    let nama = "Bima";
    println!("Nama: {}", nama);

    let mut full_name = String::from("Bima");
    full_name.push_str(" Adam");
    println!("Full name: {}", full_name);

    // shadowing
    let x = 2;
    let x = x * 10;
    println!("Shadowing x: {}", x);
}