fn triangle(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn pentagonal(n: i64) -> i64 {
    n * (3 * n - 1) / 2
}

fn hexagonal(n: i64) -> i64 {
    n * (2 * n - 1)
}

pub fn solve() -> i64 {
    let (mut ti, mut pi, mut hi) = (286, 166, 144);
    let (mut tn, mut pn, mut hn) = (triangle(ti), pentagonal(pi), hexagonal(hi));

    while hn != tn || hn != pn {
        if tn < hn {
            ti += 1;
            tn = triangle(ti);
        }

        if pn < hn {
            pi += 1;
            pn = pentagonal(pi);
        }

        if hn < tn || hn < pn {
            hi += 1;
            hn = hexagonal(hi);
        }
    }

    tn
}
