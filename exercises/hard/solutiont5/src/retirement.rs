use std::collections::HashMap;
use std::sync::LazyLock;

// 延迟退休策略开始年份
static DELAY_STRATEGY_START_YEAR: LazyLock<i32> = LazyLock::new(|| 2025);
// 延迟退休策略
// 键为职工类型，值为元组(原法定退休年龄, 延迟比例, 最大延迟月数)
// 男职工: 原法定退休年龄60周岁, 延迟比例4, 最大延迟月数36
// 原法定退休年龄50周岁女职工: 原法定退休年龄50周岁, 延迟比例2, 最大延迟月数60
// 原法定退休年龄55周岁女职工: 原法定退休年龄55周岁, 延迟比例4, 最大延迟月数36
static DELAY_STRATEGY: LazyLock<HashMap<String, (i32, i32, i32)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("男职工".to_string(), (60, 4, 36));
    map.insert("原法定退休年龄50周岁女职工".to_string(), (50, 2, 60));
    map.insert("原法定退休年龄55周岁女职工".to_string(), (55, 4, 36));
    map
});

fn parse_date(date_str: &str) -> (i32, u32) {
    let parts: Vec<&str> = date_str.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    (year, month)
}

fn add_months(date: (i32, u32), months: i32) -> (i32, u32) {
    let mut year = date.0;
    let mut month = date.1 as i32;

    month += months;
    year += (month - 1) / 12;
    month = (month - 1) % 12 + 1;

    (year, month as u32)
}

fn calculate_delay_months(
    birth_date: (i32, u32),
    original_retirement_age: i32,
    delay_ratio: i32,
    max_delay_months: i32,
) -> i32 {
    let retirement_year = birth_date.0 + original_retirement_age;
    let delay_start_year = *DELAY_STRATEGY_START_YEAR;

    if retirement_year < delay_start_year {
        return 0;
    }

    let years_after_2025 = retirement_year - delay_start_year;
    let months_after_2025 = years_after_2025 * 12 + (birth_date.1 as i32 + 3);

    (months_after_2025 / delay_ratio).min(max_delay_months)
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_date = parse_date(time);
    let person_type = tp;

    let (original_retirement_age, delay_ratio, max_delay_months) = DELAY_STRATEGY
        .get(person_type)
        .ok_or("Invalid person type")
        .copied()
        .unwrap();

    let delay_months = calculate_delay_months(
        birth_date,
        original_retirement_age,
        delay_ratio,
        max_delay_months,
    );
    let retirement_age = original_retirement_age as f64 + delay_months as f64 / 12.0;
    let retirement_date = add_months(birth_date, original_retirement_age * 12 + delay_months);

    format!(
        "{}-{:02},{},{}",
        retirement_date.0,
        retirement_date.1,
        format!("{:.2}", retirement_age).replace(".00", ""),
        delay_months
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1971_04() {
        assert_eq!(
            retire_time("1971-04", "原法定退休年龄55周岁女职工"),
            "2026-08,55.33,4"
        );
    }

    #[test]
    fn test_1965_01() {
        assert_eq!(retire_time("1965-01", "男职工",), "2025-02,60.08,1");
    }
}
