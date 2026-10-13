# Tahap 8 - Ownership dan Borrowing

Ownership adalah aturan Rust untuk mengelola memori dengan aman.

```rust
let nama = String::from("Budi");
let nama_lain = nama;
println!("{}", nama_lain);
```

Setelah dipindahkan ke `nama_lain`, `nama` tidak boleh digunakan lagi. Ini
disebut *move*.

Agar fungsi hanya meminjam data, gunakan reference:

```rust
fn tampilkan_nama(nama: &str) {
    println!("Nama: {}", nama);
}

fn main() {
    let nama = String::from("Budi");
    tampilkan_nama(&nama);
    println!("{}", nama);
}
```

Pelajari secara bertahap:

- ownership
- move
- borrowing
- reference
- mutable reference
- slice

Pesan error dari borrow checker adalah petunjuk untuk memperbaiki aturan
kepemilikan data, bukan sesuatu yang perlu dihindari.

## Latihan

Buat fungsi yang menerima `&Vec<i32>` untuk membaca data dan fungsi lain yang
menerima `&mut Vec<i32>` untuk menambahkan data.
