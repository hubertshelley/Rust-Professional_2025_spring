pub fn dp_rec_mc(mut amount: u32) -> u32 {
    // 总金额为0时，不需要任何硬币，返回0
    if amount == 0 {
        return 0;
    }
    let money_list = [100, 50, 30, 20, 10, 5, 2, 1];
    // 如果刚好和面额相等，则只需要1枚硬币
    if money_list.contains(&amount) {
        return 1;
    }
    let mut result = 0;
    for &coin in money_list.iter() {
        result += amount / coin;
        amount %= coin;
        if amount == 0 {
            break;
        }
    }
    result
}
