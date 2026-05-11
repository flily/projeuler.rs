use rustc_hash::FxHashMap;

fn get_divisors_sum(n: i64) -> i64 {
    let mut sum = 1;
    let mut i = 2;
    while i * i < n {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }

    sum
}

const LIMIT: i64 = 10_000;

pub fn solve() -> i64 {
    let mut result = 0;
    let mut set = FxHashMap::default();

    for n in 2..LIMIT {
        let s = get_divisors_sum(n);
        set.insert(n, s);
        if s != n && set.contains_key(&s) && set[&s] == n {
            result += n + s;
        }
    }

    result
}
