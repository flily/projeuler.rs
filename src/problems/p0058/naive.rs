fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
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

struct DiagonalIterator {
    n: i64,
    c: i64,
    d: i64,
}

impl DiagonalIterator {
    fn new() -> Self {
        Self { n: -1, c: -1, d: 2 }
    }
}

impl Iterator for DiagonalIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += self.d;
        self.c += 1;
        
        if self.c == 4 {
            self.c = 0;
            self.d += 2;
        }

        Some(self.n)
    }
}


pub fn solve() -> i64 {
    let mut primes = 0;
    let mut count = 0;

    let diagonal = DiagonalIterator::new();
    for n in diagonal {
        count += 1;
        if is_prime(n) {
            primes += 1;
        }

        if count > 1 && count % 4 == 1 && primes * 10 < count {
            break;
        }
    }

    (count + 1) / 2
}
