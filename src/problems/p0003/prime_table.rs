static NUMBER: i64 = 600851475143;

struct PrimeTable {
    table: Vec<i64>,
    largest: i64,
}

static SMALL_PRIMES: [i64; 10] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

impl PrimeTable {
    fn new(size: usize) -> Self {
        let mut t = Vec::with_capacity(size);
        for p in &SMALL_PRIMES {
            t.push(*p);
        }

        Self {
            table: t,
            largest: 31,
        }
    }

    fn is_prime(&mut self, n: i64) -> bool {
        if n <= 2 {
            return true;
        }

        if n < self.largest {
            return self.table.binary_search(&n).is_ok();
        }

        for p in &self.table {
            if p * p > n {
                break;
            }

            if n % p == 0 {
                return false;
            }
        }

        self.table.push(n);
        self.largest = n;

        true
    }
}

fn remove_factor(n: i64, f: i64) -> i64 {
    let mut m = n;
    while m % f == 0 {
        m /= f;
    }

    m
}

pub fn solve() -> i64 {
    let mut table = PrimeTable::new(1000);

    let mut last = 1i64;
    let mut n = NUMBER;
    let mut i = 3i64;
    while n > 0 && i <= n {
        if table.is_prime(i) && n % i == 0 {
            last = i;
            n = remove_factor(n, i);
        }

        i += 2;
    }

    last
}
