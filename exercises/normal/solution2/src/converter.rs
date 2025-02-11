pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let num_str: Vec<_> = num_str.split(&['(', ')'][..]).collect();
    let (radix_str, from_base): (&str, u32) = (num_str[0], num_str[1].parse().unwrap());
    let mut num = i64::from_str_radix(radix_str, from_base).unwrap();
    let mut result = String::new();
    while num != 0 {
        let cur_num = (num % to_base as i64) as u8;
        result.insert(
            0,
            ({
                if cur_num > 10 {
                    b'a' + (cur_num - 10)
                } else {
                    cur_num + b'0'
                }
            }) as char,
        );
        num /= to_base as i64;
    }
    result
}
