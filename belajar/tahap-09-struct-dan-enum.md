# Tahap 9 - Struct dan Enum

`struct` digunakan untuk membuat tipe data sendiri:

```rust
struct Barang {
    nama: String,
    harga: i32,
    stok: i32,
}

fn main() {
    let barang = Barang {
        nama: String::from("Buku"),
        harga: 5000,
        stok: 10,
    };

    println!("{} - Rp{}", barang.nama, barang.harga);
}
```

`enum` digunakan untuk mewakili beberapa kemungkinan:

```rust
enum Status {
    Aktif,
    Selesai,
    Gagal,
}
```

`Option` dan `Result` adalah enum penting yang akan sering digunakan saat
program menangani data yang mungkin kosong atau operasi yang mungkin gagal.

## Latihan

Buat `struct Siswa` dan `enum StatusTugas`, lalu tampilkan data keduanya.
