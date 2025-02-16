use std::iter::Iterator;
use std::ops::Sub;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

static TOTAL_DAYS: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut total = 0;
    (1..=2999)
        .map(|y| {
            total += if Date::new(y, 1, 1).is_leap_year() {
                366
            } else {
                365
            };
            total
        })
        .collect()
});

impl Date {
    fn new(year: i32, month: i32, day: i32) -> Self {
        Date { year, month, day }
    }

    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    fn day_of_year(&self) -> i32 {
        let mut days = self.day;
        for m in 1..self.month {
            days += self.days_in_month(m);
        }
        days
    }

    fn year_days(&self) -> i32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn total_days(&self) -> i32 {
        if self.year < TOTAL_DAYS.len() as i32 {
            TOTAL_DAYS[self.year as usize - 1] + self.day_of_year()
        } else {
            let mut total = *TOTAL_DAYS.last().unwrap();
            let start = TOTAL_DAYS.len() as i32;
            for y in start..self.year {
                total += if Date::new(y, 1, 1).is_leap_year() {
                    366
                } else {
                    365
                };
            }
            total + self.day_of_year()
        }
    }
    fn weekday(&self) -> i32 {
        let weekday = (self.total_days() + 6) % 7;
        if weekday == 0 {
            7
        } else {
            weekday
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_days().cmp(&other.total_days())
    }
}

impl Sub for Date {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        self.total_days() - other.total_days()
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid date format: {}", s));
        }
        let year = parts[0]
            .parse()
            .map_err(|e| format!("Invalid year: {}", e))?;
        let month = parts[1]
            .parse()
            .map_err(|e| format!("Invalid month: {}", e))?;
        let day = parts[2]
            .parse()
            .map_err(|e| format!("Invalid day: {}", e))?;
        Ok(Date::new(year, month, day))
    }
}

static SPRING_FESTIVAL: LazyLock<Vec<Date>> =
    LazyLock::new(|| vec![Date::new(2025, 1, 29), Date::new(2026, 2, 17)]);

static A_2025_HOLIDAY: LazyLock<Vec<(Date, Date)>> = LazyLock::new(|| {
    vec![
        (Date::new(2025, 1, 1), Date::new(2025, 1, 1)),
        (Date::new(2025, 1, 28), Date::new(2025, 2, 4)),
        (Date::new(2025, 4, 4), Date::new(2025, 4, 6)),
        (Date::new(2025, 5, 1), Date::new(2025, 5, 6)),
        (Date::new(2025, 5, 31), Date::new(2025, 6, 2)),
        (Date::new(2025, 10, 1), Date::new(2025, 10, 8)),
        (Date::new(2026, 1, 1), Date::new(2026, 1, 1)),
    ]
});

fn next_a_start(date: Date) -> i32 {
    for (start, end) in A_2025_HOLIDAY.iter() {
        if start.total_days() <= date.total_days() + 1 && &date <= end {
            return *end - date;
        }
    }
    if date.weekday() == 5 {
        2
    } else if date.weekday() == 6 {
        1
    } else {
        0
    }
}

pub fn time_info(time: &str) -> String {
    let date: Date = time.parse().unwrap();
    let days_until_spring_festival = SPRING_FESTIVAL
        .iter()
        .find(|&d| d > &date)
        .copied()
        .unwrap()
        - date;
    let weekday = date.weekday();
    let week_number = if Date::new(date.year, 1, 1).weekday() == 0 {
        date.day_of_year() / 7
    } else {
        date.day_of_year() / 7 + 1
    };
    format!(
        "{},{},{},{},{},{}",
        week_number,
        weekday,
        date.day_of_year(),
        date.year_days() - date.day_of_year(),
        days_until_spring_festival,
        next_a_start(date)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {
        let d1 = Date::new(2021, 1, 1);
        let d2 = Date::new(2021, 1, 2);
        let today = Date::new(2025, 2, 16);
        let leap_year = Date::new(2000, 1, 2);
        assert_eq!(d2 - d1, 1);
        assert_eq!(d1 - d2, -1);
        assert_eq!(today.weekday(), 0);
        assert_eq!(today.year_days(), 365);
        assert_eq!(leap_year.year_days(), 366);
        assert_eq!(d1.day_of_year(), 1);
        assert_eq!(d2.day_of_year(), 2);
        assert!(Date::new(2020, 2, 29).is_leap_year());
        assert!(Date::new(2024, 2, 29).is_leap_year());
        assert_eq!(Date::new(2020, 2, 29).days_in_month(2), 29);
        assert_eq!(Date::new(2021, 2, 28).days_in_month(2), 28);
        assert_eq!(Date::new(2020, 2, 29).total_days(), 737850);
        assert_eq!(Date::new(2021, 2, 28).total_days(), 738214);
        assert_eq!(Date::new(2020, 2, 29).day_of_year(), 60);
        assert_eq!(Date::new(2021, 2, 28).day_of_year(), 59);
        assert_eq!(Date::new(2025, 1, 29) - Date::new(2021, 1, 1), 1489);
        assert_eq!(Date::new(2026, 2, 17) - Date::new(2021, 1, 1), 1873);
    }

    #[test]
    fn test_2025_latest() {
        let date: Date = "2025-12-31".parse().unwrap();
        let week_number = if Date::new(date.year, 1, 1).weekday() == 0 {
            date.day_of_year() / 7
        } else {
            date.day_of_year() / 7 + 1
        };
        assert_eq!(week_number, 53);
    }
}
