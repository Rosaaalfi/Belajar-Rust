use std::io::{self, Write};

fn main() {
    // Membuat variabel mutable untuk menyimpan input dari pengguna
    let mut sisi1 = String::new();
    let mut sisi2 = String::new();

    // Prompt pengguna untuk memasukkan sisi-sisi persegi panjang
    print!("Masukkan sisi 1: ");
    // Flush stdout untuk memastikan prompt ditampilkan sebelum membaca input
    io::stdout().flush().unwrap();
    // Baca input dari pengguna dan simpan ke variabel sisi1
    io::stdin()
        .read_line(&mut sisi1)
        .expect("Gagal membaca input");
    print!("Masukkan sisi 2: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut sisi2)
        .expect("Gagal membaca input"); 

    // Konversi input dari String ke f64 dan tangani kemungkinan kesalahan konversi
    let sisi1: f64 = sisi1.trim().parse().expect("Gagal mengonversi sisi 1");
    let sisi2: f64 = sisi2.trim().parse().expect("Gagal mengonversi sisi 2");

    println!("Hasil penjumlahan: {}", sisi1 * sisi2);
}

// Latihan Variable
//fn main() {
//    Variabel adalah tempat untuk menyimpan data. Variabel dapat memiliki tipe data yang berbeda, seperti integer, float, string, dan lain-lain. Variabel juga dapat diubah nilainya selama program berjalan, tergantung pada kebutuhan program. Berikut adalah contoh penggunaan variabel dalam bahasa pemrograman Rust:
//    let nama = "Aniko";
//    let umur = 21;
//    let hobi = "Gaming";
//    let impian = "Menjadi software engineer dan reverse engineer";
//    let tinggi = 160.0;
//
//    print!(
//        "Nama: {} \nUmur: {} \nTinggi: {} \nHobi: {} \nImpian: {}",
//        nama, umur, tinggi, hobi, impian
//    );
//}
