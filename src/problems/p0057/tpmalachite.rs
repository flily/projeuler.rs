use malachite::Integer;

struct Fraction {
    numerator: Integer,
    denominator: Integer,
}

impl Fraction {
    fn new(n: i64, d: i64) -> Self {
        Self {
            numerator: Integer::from(n),
            denominator: Integer::from(d),
        }
    }

    fn add(&self, other: &Fraction) -> Self {
        let numerator = &self.numerator * &other.denominator
            + &other.numerator * &self.denominator;
        let denominator = &self.denominator * &other.denominator;
        Self { numerator, denominator }
    }

    fn div(&self, other: &Fraction) -> Self {
        let numerator = &self.numerator * &other.denominator;
        let denominator = &self.denominator * &other.numerator;
        Self { numerator, denominator }
    }
}

fn expand(n: i64) -> Fraction {
    let one = Fraction::new(1, 1);
    let two = Fraction::new(2, 1);
    let half = Fraction::new(1, 2);

    if n == 1 {
        return one.add(&half);
    }

    let mut result = one.div(&two.add(&half));
    let mut m = n;
    while m > 1 {
        result = one.div(&two.add(&result));
        m -= 1;
    }

    result.add(&one)
}

pub fn solve() -> i64 {
    let mut count = 0;
    for i in 1..1000 {
        let fraction = expand(i);
        if fraction.numerator.to_string().len() > fraction.denominator.to_string().len() {
            count += 1;
        }
    }

    count
}

pub fn solve_generator() -> i64 {
    let mut count = 0;
    let one = Fraction::new(1, 1);
    let two = Fraction::new(2, 1);
    let half = Fraction::new(1, 2);

    let mut n = half;
    for _ in 1..1000 {
        n = one.div(&two.add(&n));

        let m = n.add(&one);
        if m.numerator.to_string().len() > m.denominator.to_string().len() {
            count += 1;
        }
    }

    count
}

pub fn solve_directly() -> i64 {
    let mut count = 0;
    let two = Integer::from(2);
    let mut n = Integer::from(2);
    let mut d = Integer::from(5);

    for _ in 1..1000 {
        let nd = &d + &n;
        if nd.to_string().len() > d.to_string().len() {
            count += 1;
        }

        (d, n) = ((&d * &two) + &n, d);
    }

    count
}

