// Generator simpel untuk menghasilkan angka acak menggunakan algoritma Xorshift.
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0x12345678 } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        self.state = x;
        x
    }

    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        assert!(min < max, "min harus lebih kecil dari max");

        min + self.next_u32() % (max - min)
    }
}
