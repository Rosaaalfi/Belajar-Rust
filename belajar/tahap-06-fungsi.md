# Tahap 6 - Fungsi

Fungsi membantu memecah program menjadi bagian-bagian kecil.

```rust
fn sapa(nama: &str) {
    println!("Halo, {}!", nama);
}

fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    sapa("Budi");
    let hasil = tambah(10, 5);
    println!("Hasil: {}", hasil);
}
```

Tipe return ditulis setelah `->`. Ekspresi terakhir dapat menjadi nilai return
tanpa kata `return`.

## Latihan

Buat fungsi untuk menghitung luas persegi, mencari nilai terbesar, dan
menghitung rata-rata.
