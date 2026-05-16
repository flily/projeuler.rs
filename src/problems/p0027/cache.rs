use rustc_hash::FxHashSet;

struct Cache {
    primes: FxHashSet<i64>,
    max: i64,
}

impl Cache {
    fn new() -> Self {
        let mut primes = FxHashSet::default();
        primes.insert(2);
        Cache { primes, max: 2 }
    }

    fn is_prime(&mut self, n: i64) -> bool {
        if n < self.max {
            return self.primes.contains(&n);
        }

        let r = is_prime(n);
        if r && n > self.max {
            self.primes.insert(n);
            self.max = n;
        }

        r
    }
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn f(a: i64, b: i64, n: i64) -> i64 {
    n * n + a * n + b
}

fn consecutive_prime_size(cache: &mut Cache, a: i64, b: i64) -> i64 {
    let mut x = 0;
    while cache.is_prime(f(a, b, x)) {
        x += 1;
    }

    x
}

pub fn solve() -> i64 {
    let mut max_prime_size = 0;
    let mut max_a = 0;
    let mut max_b = 0;
    let mut cache = Cache::new();

    for a in -999..1000 {
        for b in -1000..1001 {
            if (b != 0 && a % b == 0) || (a != 0 && b % a == 0) {
                continue;
            }

            let size = consecutive_prime_size(&mut cache, a, b);
            if size > max_prime_size {
                max_prime_size = size;
                (max_a, max_b) = (a, b);
            }
        }
    }   

    max_a * max_b
}
