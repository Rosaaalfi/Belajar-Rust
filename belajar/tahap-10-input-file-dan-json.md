# Tahap 10 - Input, File, dan JSON

## Membaca input terminal

```rust
use std::io;

fn main() {
    let mut nama = String::new();

    println!("Masukkan nama:");
    io::stdin()
        .read_line(&mut nama)
        .expect("Gagal membaca input");

    println!("Halo, {}", nama.trim());
}
```

## Membaca JSON proyek

File [barang.json](../src/data/barang.json) memiliki object `barang` yang berisi
array. Gunakan `struct` agar bentuk data jelas:

```rust
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct DataBarang {
    barang: Vec<Barang>,
}

#[derive(Debug, Deserialize)]
struct Barang {
    id: i32,
    nama: String,
    harga: i32,
    stok: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let isi_file = fs::read_to_string("src/data/barang.json")?;
    let data: DataBarang = serde_json::from_str(&isi_file)?;

    for barang in data.barang {
        println!(
            "{}. {} - Rp{} - stok {}",
            barang.id, barang.nama, barang.harga, barang.stok
        );
    }

    Ok(())
}
```

Kode menggunakan `serde` dan `serde_json` yang sudah tersedia di
`Cargo.toml`. Tanda `?` meneruskan error kepada pemanggil.

## Latihan

Baca daftar barang, cari barang berdasarkan nama, dan hitung nilai total stok.
