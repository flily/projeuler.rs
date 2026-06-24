use rustc_hash::{FxHashSet, FxHashMap};

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let mut m = 3;
    while m * m <= n {
        if n % m == 0 {
            return false;
        }

        m += 2;
    }

    true
}

fn log10_base(n: i64) -> i64 {
    let mut base_next = 10;
    while n >= base_next {
        base_next *= 10;
    }

    base_next
}

const TOTAL_NUMS: usize = 5;

fn get_main_primes(limit: i64) -> Vec<i64> {
    let mut result = Vec::new();

    // skip 2
    for x in (3..limit).step_by(2) {
        if x % 5 == 0 {
            continue;
        }

        if is_prime(x) {
            result.push(x);
        }
    }

    result
}

fn find_prime_pair_set(prime_pairs: &FxHashSet<(i64, i64)>, primes: &[i64], prime_index: usize, state: &mut [i64], state_index: usize) -> Option<i64> {
    if state_index == state.len() {
        let sum: i64 = state.iter().sum();
        Some(sum)

    } else {
        let mut i = prime_index;
        while i < primes.len() {
            let p_next = primes[i];
            let mut found = true;
            for &p in state.iter().take(state_index) {
                if !prime_pairs.contains(&(p, p_next)) {
                    found = false;
                    break;
                }
            }

            if found {
                state[state_index] = p_next;
                let r = find_prime_pair_set(prime_pairs, primes, i + 1, state, state_index + 1);
                if r.is_some() {
                    return r;
                }
            }

            i += 1;
        }

        None
    }
}

pub fn solve() -> i64 {
    let main_primes = get_main_primes(10_000);

    let mut prime_pairs = FxHashSet::default();
    let mut prime_pairs_map = FxHashMap::default();
    for i in 0..main_primes.len() {
        for j in (i+1)..main_primes.len() {
            let a = main_primes[i];
            let b = main_primes[j];

            let base_a = log10_base(a);
            let base_b = log10_base(b);

            let ab = a * base_b + b;
            let ba = b * base_a + a;

            if is_prime(ab) && is_prime(ba) {
                prime_pairs.insert((a, b));
                prime_pairs_map.entry(a).or_insert_with(Vec::new).push(b);
                prime_pairs_map.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
    }

    let mut possible_primes = FxHashSet::default();
    for primes in prime_pairs_map.values() {
        if primes.len() >= TOTAL_NUMS - 1 {
            for &p in primes {
                possible_primes.insert(p);
            }
        }
    }

    let mut possible_prime_list = Vec::new();
    let mut possible_prime_map = FxHashMap::default();
    for (&k, v) in prime_pairs_map.iter() {
        if !possible_primes.contains(&k) {
            continue;
        }

        let mut prime_list = Vec::new();
        for &p in v {
            if p >= k {
                prime_list.push(p);
            }
        }
        prime_list.push(k);
        prime_list.sort();
        possible_prime_map.insert(k, prime_list);
        possible_prime_list.push(k);
    }
    possible_prime_list.sort();

    let mut state = vec![0; TOTAL_NUMS];
    find_prime_pair_set(&prime_pairs, &possible_prime_list, 0, &mut state, 0).unwrap()
}
