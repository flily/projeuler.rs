use num_bigint::BigInt;

pub fn solve() -> i64 {
    let a = BigInt::from(1);
    let b = BigInt::from(1);
    let mut fibos = vec![a, b];

    while fibos.last().unwrap().to_string().len() < 1000 {
        let length = fibos.len();
        let c = &fibos[length - 1] + &fibos[length - 2];
        fibos.push(c);
    }

    fibos.len() as i64
}

pub fn solve_no_array() -> i64 {
    let mut a = BigInt::from(1);
    let mut b = BigInt::from(1);
    let mut count = 2;
    while b.to_string().len() < 1000 {
        let c = a + &b;
        a = b;
        b = c;
        count += 1;
    }

    count
}