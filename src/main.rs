mod rng;

use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use rng::Rng;
use std::cmp::Ordering;

fn main() {
    // Ambil seed dari waktu saat ini untuk menghasilkan angka acak yang berbeda setiap kali program dijalankan
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u32;
    let mut tebak = String::new();
    let mut rng = Rng::new(seed);
    let jawaban = rng.range(1, 101);

    println!("Tebak angka!");
    println!("Masukkan angka tebakan: ");

    // Baca input tebakan dari pemain
    io::stdin().read_line(&mut tebak).expect("X");

    println!("Tebakan: {}", tebak);

    // Konversi input tebakan menjadi tipe data yang sesuai
    let tebak: i32 = tebak.trim().parse().expect("X");
    let jawaban: i32 = jawaban as i32;

    // Periksa tebakan dan berikan feedback kepada pemain
    match tebak.cmp(&jawaban) {
        Ordering::Less if (jawaban - tebak).abs() <= 2 => println!("Tebakanmu hampir benar!, Jawaban: {}, Tebakan: {}", jawaban, tebak),
        Ordering::Less => println!("Tebakanmu terlalu kecil!, Jawaban: {}, Tebakan: {}", jawaban, tebak),
        Ordering::Greater if (tebak - jawaban).abs() <= 2 => println!("Tebakanmu hampir benar!, Jawaban: {}, Tebakan: {}", jawaban, tebak),
        Ordering::Greater => println!("Tebakanmu terlalu besar!, Jawaban: {}, Tebakan: {}", jawaban, tebak),
        Ordering::Equal => println!("Selamat! Tebakanmu benar., Jawaban: {}, Tebakan: {}", jawaban, tebak)
    }
}
