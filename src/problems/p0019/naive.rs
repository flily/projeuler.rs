const DAYS_IN_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn solve() -> i64 {
    let mut result = 0;
    let mut days = 365; // count from 1901-01-01

    for year in 1901..=2000 {
        for (month, month_days) in DAYS_IN_MONTH.iter().enumerate() {
            // 1900-01-01 is Monday, so Sunday is 6 days after
            if days % 7 == 6 {
                result += 1;
            }

            days += month_days;
            if month == 1 && is_leap_year(year) {
                days += 1;
            }
        }
    }

    result
}
