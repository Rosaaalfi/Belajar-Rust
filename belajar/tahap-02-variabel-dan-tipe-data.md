# Tahap 2 - Variabel dan Tipe Data

```rust
fn main() {
    let nama = "Budi";
    let umur: i32 = 20;
    let tinggi: f64 = 170.5;
    let sudah_belajar = true;

    println!("Nama: {}", nama);
    println!("Umur: {}", umur);
    println!("Tinggi: {}", tinggi);
    println!("Sudah belajar: {}", sudah_belajar);
}
```

Tipe dasar yang sering digunakan:

- `&str` dan `String` untuk teks.
- `i32` untuk bilangan bulat.
- `f64` untuk bilangan desimal.
- `bool` untuk `true` atau `false`.
- `char` untuk satu karakter.

Variabel tidak dapat diubah secara default. Gunakan `mut` jika perlu:

```rust
let mut umur = 20;
umur = 21;
```

## Latihan

Buat program untuk menghitung luas persegi panjang.
