fn sieve_totient(max_num: i64) -> i64 {
    let mut sieve = vec![0; (max_num + 1) as usize];
    let mut result = 0;

    for i in 2..=(max_num as usize) {
        if sieve[i] == 0 {
            for j in (i..=(max_num as usize)).step_by(i) {
                if sieve[j] == 0 {
                    sieve[j] = j;
                }
                sieve[j] = sieve[j] / i * (i - 1);
            }
        }

        result += sieve[i] as i64;
    }

    result
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    sieve_totient(LIMIT)
}
