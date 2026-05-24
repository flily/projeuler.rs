use rustc_hash::FxHashSet;

fn find_right_triangle_solutions(max_p: i64) -> i64 {
    let mut results = vec![FxHashSet::default(); (max_p + 1) as usize];

    for n in 1..(max_p/2) {
        for m in (n + 1)..(max_p/n) {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            let p = a + b + c;
            if p > max_p {
                break;
            }

            let mut k = 1;
            while k * p <= max_p {
                let sides = if a < b {
                    (k * b, k * a, k * c)
                } else {
                    (k * a, k * b, k * c)
                };

                results[(k * p) as usize].insert(sides);
                k += 1;
            }
        }
    }

    let mut result = 0;
    let mut max_count = 0;
    for p in 1..=max_p {
        let count = results[p as usize].len() as i64;
        if count > max_count {
            max_count = count;
            result = p;
        }
    }

    result
}


pub fn solve() -> i64 {
    find_right_triangle_solutions(1000)
}
