fn get_cycle_length(n: i64) -> i64 {
    let base = (n as f64).log(10.0).ceil() as i64;
    let mut m = base;
    let mut digits = Vec::new();
    let mut loop_start = -1;

    while m != 0 {
        let r = m / n;
        let d = m % n;
        if d == 0 {
            break;
        }

        let index = (r, d);
        if let Some(pos) = digits.iter().position(|&x| x == index) {
            loop_start = pos as i64;
            break;
        }

        digits.push(index);
        m = d;
        while m < n {
            m *= 10;
        }
    }

    if loop_start < 0 {
        0
    } else {
        digits.len() as i64 - loop_start
    }
}

pub fn solve() -> i64 {
    let mut max_cycle_length = 0;
    let mut max_cycle_number = 0;

    for i in 2..1000 {
        let l = get_cycle_length(i);
        if l > max_cycle_length {
            max_cycle_length = l;
            max_cycle_number = i;
        }
    }

    max_cycle_number
}

fn is_prime(n: i64) -> bool {
    // assume n > 3 and n is odd
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

pub fn solve_prime() -> i64 {
    let mut max_cycle_length = 0;
    let mut max_cycle_number = 0;

    for i in (3..1000).step_by(2) {
        if is_prime(i) {
            let l = get_cycle_length(i);
            if l > max_cycle_length {
                max_cycle_length = l;
                max_cycle_number = i;
            }
        }
    }

    max_cycle_number
}
