use rustc_hash::FxHashMap;

fn num_key(n: i64) -> [i64; 10] {
    let mut count = [0; 10];
    let mut x = n;
    while x > 0 {
        count[(x % 10) as usize] += 1;
        x /= 10;
    }
    count
}

pub fn solve() -> i64 {
    let mut digit_map: FxHashMap<[i64; 10], Vec<i64>> = FxHashMap::default();

    for x in 1..10000 {
        let cube = x * x * x;
        let key = num_key(cube);
        if let Some(cubes) = digit_map.get_mut(&key) {
            cubes.push(cube);
            if cubes.len() == 5 {
                return cubes[0];
            }
        } else {
            let item = vec![cube];
            digit_map.insert(key, item);
        }
    }

    0
}
