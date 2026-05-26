fn can_be_right_triangle(a: i64, b: i64, c: i64) -> bool {
    let aa = a * a;
    let bb = b * b;
    let cc = c * c;
    aa + bb == cc || aa + cc == bb || bb + cc == aa
}

fn find_right_triangle_solutions(p: i64) -> i64 {
    let mut count = 0;

    for a in 1..(p / 2) {
        for b in a..((p - a) / 2) {
            let c = p - a - b;
            if c <= 0 {
                break;
            }

            if can_be_right_triangle(a, b, c) {
                count += 1;
            }
        }
    }

    count
}

const LIMIT: i64 = 1000;

pub fn solve() -> i64 {
    let mut result = 0;
    let mut max_count = 0;
    for p in 1..=LIMIT {
        let count = find_right_triangle_solutions(p);
        if count > max_count {
            max_count = count;
            result = p;
        }
    }

    result
}
