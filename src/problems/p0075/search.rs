use rustc_hash::FxHashMap;

fn find_pythagorean_c(a: i64, b: i64) -> i64 {
    let cc = a * a + b * b;
    let c = (cc as f64).sqrt() as i64;
    if c * c == cc {
        c
    } else {
        -1
    }
}

static LIMIT: i64 = 1_500_000;

pub fn solve() -> i64 {
    let mut nums = FxHashMap::default();

    for a in 1..LIMIT / 2 {
        for b in a..LIMIT {
            let c = find_pythagorean_c(a, b);
            if c <= 0 {
                continue;
            }

            let l = a + b + c;
            if l > LIMIT {
                break;
            }


            if nums.contains_key(&l) {
                nums.insert(l, nums[&l] + 1);
            } else {
                nums.insert(l, 1);
            }
        }
    }

    let mut count = 0;
    for (_, v) in nums {
        if v == 1 {
            count += 1;
        }
    }

    count
}
