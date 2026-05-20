fn collatz_seq_size(cache: &mut [i64], n: i64) -> i64 {
    let mut m = n;
    let mut result = 1;

    while m > 1 {
        let item = if m < cache.len() as i64 {
            cache[m as usize]
        } else {
            0
        };

        if 0 == item {
            m = if m % 2 == 0 { m / 2 } else { 3 * m + 1 };
            result += 1;
        } else {
            result += item;
            break;
        }
    }

    cache[n as usize] = result;
    result
}

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i64 {
    let mut cache = vec![0; (LIMIT + 1) as usize];
    cache[0] = 1;
    cache[1] = 1;
    cache[2] = 2;
    cache[3] = 9;
    cache[4] = 4;
    let mut max_size = 0;
    let mut result = 0;

    for i in 1..LIMIT {
        let size = collatz_seq_size(&mut cache, i);
        if size > max_size {
            max_size = size;
            result = i;
        }
    }

    result
}
