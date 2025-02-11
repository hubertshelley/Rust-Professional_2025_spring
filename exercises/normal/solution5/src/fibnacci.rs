pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold <= 1 {
        return 0;
    }
    let mut a = 1;
    let mut b = 1;
    let mut sum = a;
    while b < threshold {
        if b % 2 != 0 {
            sum += b;
        }
        (a, b) = (b, a + b);
    }
    sum
}
