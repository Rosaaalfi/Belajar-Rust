# Tahap 4 - Percabangan

```rust
fn main() {
    let nilai = 80;

    if nilai >= 75 {
        println!("Lulus");
    } else {
        println!("Tidak lulus");
    }
}
```

Untuk banyak kondisi:

```rust
let nilai = 85;

if nilai >= 90 {
    println!("A");
} else if nilai >= 80 {
    println!("B");
} else {
    println!("Perlu belajar lagi");
}
```

`match` cocok untuk pilihan yang jelas:

```rust
let pilihan = 2;

match pilihan {
    1 => println!("Tambah data"),
    2 => println!("Lihat data"),
    3 => println!("Keluar"),
    _ => println!("Pilihan tidak dikenal"),
}
```

## Latihan

Buat program untuk menentukan bilangan ganjil/genap dan menentukan nilai A
sampai D.
