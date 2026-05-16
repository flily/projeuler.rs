use malachite::Integer;

pub fn solve() -> i64 {
    let mut a = Integer::from(1);
    let mut b = Integer::from(1);
    let mut count = 2;
    while b.to_string().len() < 1000 {
        let c = a + &b;
        a = b;
        b = c;
        count += 1;
    }

    count
}
