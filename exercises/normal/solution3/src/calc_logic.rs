pub fn new_birthday_probability(n: u32) -> f64 {
    1f64 - (0..n).map(|n| ((365 - n) as f64) / 365f64).product::<f64>()
}
