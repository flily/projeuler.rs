fn num_digits(n: i128) -> i64 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    let mut m = n;
    while m > 0 {
        m /= 10;
        count += 1;
    }

    count
}

fn find_n_power_numbers(n: i64) -> i64 {
    let mut count = 0;

    let mut m = 1_i128;
    loop {
        let num = m.pow(n as u32);
        let digits = num_digits(num);
        if digits == n {
            count += 1;
        
        } else if digits > n {
            break;
        }

        m += 1;
    }

    count
}


pub fn solve() -> i64 {
    let mut count = 0;
    for n in 1.. {
        count += find_n_power_numbers(n);
        if find_n_power_numbers(n) == 0 {
            break;
        }
    }

    count
}
