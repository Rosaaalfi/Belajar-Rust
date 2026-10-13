# Tahap 3 - Operator dan Perhitungan

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", a % b);
}
```

Operator perbandingan menghasilkan `bool`:

```rust
let umur = 18;
println!("{}", umur >= 17);
println!("{}", umur == 18);
println!("{}", umur != 20);
```

Operator logika yang umum adalah `&&` (dan), `||` (atau), dan `!` (negasi).

## Latihan

Buat program konversi Celsius ke Fahrenheit dan hitung total belanja beberapa
barang.
