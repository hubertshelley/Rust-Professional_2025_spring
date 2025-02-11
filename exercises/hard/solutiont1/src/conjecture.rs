pub fn goldbach_conjecture() -> String {
    let mut result = String::new();
    let mut cur_count = 0;
    let mut num = 9;
    let mut s_num = 4;
    let mut primes = vec![2, 3, 5, 7];
    let mut sqrt_nums = vec![1, 4, 9];
    let mut sqrt_count = 3i32;
    let mut sqrt_c_num = 9i32;
    while cur_count < 2 {
        num += 2;
        s_num += 1;
        if sqrt_c_num < num {
            sqrt_nums.push(sqrt_c_num);
            sqrt_count += 1;
            sqrt_c_num = sqrt_count.pow(2);
        }
        if (3..=s_num).all(|x| num % x != 0) {
            primes.push(num);
            continue;
        }
        if !primes.iter().any(|&x| sqrt_nums.contains(&((num - x) / 2))) {
            cur_count += 1;
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str(&num.to_string());
        }
    }
    result
}
