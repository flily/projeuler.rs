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

fn get_number_families(n: i64) -> ([i64; 10], [i64; 10]) {
    let mut base = [n; 10];
    let mut delta = [0; 10];
    let mut m = n;
    let mut e = 1;

    while m > 0 {
        let d = (m % 10) as usize;
        m /= 10;

        delta[d] += e;
        base[d] -= e * d as i64;
        e *= 10;
    }

    (base, delta)
}

pub fn solve() -> i64 {
    let mut n = 56993;
    loop {
        if is_prime(n) {
            let (base, delta) = get_number_families(n);
            for i in 0..10 {
                if base[i] == n {
                    continue;
                }

                let mut count = 0;
                for j in 0..10 {
                    if base[i] < delta[i] && j == 0 {
                        continue;
                    }

                    let m = base[i] + delta[i] * j;
                    if is_prime(m) {
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
