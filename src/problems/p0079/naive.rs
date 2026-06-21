use super::load;

fn check_success(passcode: i64, attempt: i64) -> bool {
    let mut p = passcode;
    let mut a = attempt;
    while p > 0 && a > 0 {
        if p % 10 == a % 10 {
            a /= 10;
        }

        p /= 10;
    }

    a == 0
}

pub fn solve() -> i64 {
    let attempts = load();
    
    for code in 100_000..100_000_000 {
        let mut success = true;
        for attempt in &attempts {
            if !check_success(code, *attempt) {
                success = false;
                break;
            }
        }

        if success {
            return code;
        }
    }

    -1
}
