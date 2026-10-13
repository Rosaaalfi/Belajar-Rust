# Tahap 7 - Koleksi dan String

## Array

Array memiliki ukuran tetap:

```rust
let angka = [10, 20, 30, 40];
println!("{}", angka[0]);
println!("{}", angka.len());
```

## Vector

`Vec` dapat bertambah ukurannya:

```rust
let mut nama = Vec::new();
nama.push("Budi");
nama.push("Siti");

for item in nama {
    println!("{}", item);
}
```

## String

```rust
let mut nama = String::from("Budi");
nama.push_str(" Santoso");
println!("{}", nama);
```

`&str` cocok untuk teks yang tidak perlu dimiliki atau diubah, sedangkan
`String` digunakan untuk teks yang dimiliki dan dapat diubah.

## Latihan

Buat daftar nama, tambahkan beberapa nama, lalu tampilkan semua isinya.
