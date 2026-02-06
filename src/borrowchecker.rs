pub fn run () {
    let mut angka = 10;
    {
        let angka_borrowed = &mut angka;
        *angka_borrowed += 5;
        println!("angka setelah di borrow dan diubah: {}", angka_borrowed);
    }  {
        println!("angka setelah scope borrow selesai: {}", angka);
    }
}
