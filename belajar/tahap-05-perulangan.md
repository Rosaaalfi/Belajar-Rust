# Tahap 5 - Perulangan

## `while`

```rust
let mut angka = 1;

while angka <= 5 {
    println!("{}", angka);
    angka += 1;
}
```

## `for`

```rust
for angka in 1..=5 {
    println!("{}", angka);
}
```

## `loop`

```rust
let mut angka = 1;

loop {
    println!("{}", angka);
    angka += 1;

    if angka > 5 {
        break;
    }
}
```

Gunakan `break` untuk berhenti dan `continue` untuk melewati satu putaran.

## Latihan

Buat program yang mencetak angka 1 sampai 100 dan menghitung total semua angka
tersebut.
