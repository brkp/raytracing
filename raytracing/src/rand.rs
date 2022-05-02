pub const RAND_MAX: u32 = 2147483647;

extern "C" {
    fn rand() -> i32;
    fn srand(seed: u32);
}

pub fn seed(seed: u32) {
    unsafe { srand(seed) }
}

pub fn f64(a: f64, b: f64) -> f64 {
    a + (b - a) * ((unsafe { rand() }) as f64 / (RAND_MAX as f64 + 1.0))
}
