pub fn run() {
    let umur: i32 = 9;

    if umur >= 8 && umur <= 17 {
        println!("Anak-anak");
    } else if umur >= 18 && umur <= 59 {
        println!("Dewasa");
    } else if umur >= 60 && umur <= 120 {
        println!("Lansia");
    } else {
        println!("Belum dewasa");
    }
}