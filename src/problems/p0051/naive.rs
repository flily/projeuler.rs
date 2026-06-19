fn is_prime(n: i64) -> bool {
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn get_number_families(n: i64) -> Vec<String> {
    let mut set_digits = [false; 10];
    let mut result = Vec::new();

    let s = n.to_string();
    for c in s.chars() {
        let cn = c as u8 - b'0';
        if set_digits[cn as usize] {
            continue;
        }

        set_digits[cn as usize] = true;
        let p = s.replace(c, "*");
        result.push(p);
    }

    result
}

const NUMBER_STRINGS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn get_family_members(family: &str) -> Vec<i64> {
    let mut result = Vec::new();
    for c in NUMBER_STRINGS {
        let s = family.replace('*', c);
        if s.starts_with('0') {
            continue;
        }

        result.push(s.parse().unwrap());
    }

    result
}

pub fn solve() -> i64 {
    let mut n = 11;
    loop {
        if is_prime(n) {
            let families = get_number_families(n);
            for family in families {
                let members = get_family_members(&family);
                let mut count = 0;
                for member in members {
                    if is_prime(member) {
                        count += 1;
                    }
                }

                if count == 8 {
                    return n;
                }
            }
        }

        n += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_families() {
        assert_eq!(get_number_families(123), vec!["*23", "1*3", "12*"]);
        assert_eq!(get_number_families(1223), vec!["*223", "1**3", "122*"]);
    }
}
