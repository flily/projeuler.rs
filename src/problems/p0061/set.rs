use rustc_hash::FxHashSet;

fn gen_triangle(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * (m + 1) / 2;
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn gen_square(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * m;
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn gen_pentagonal(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * (3 * m - 1) / 2;
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn gen_hexagonal(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * (2 * m - 1);
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn gen_heptagonal(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * (5 * m - 3) / 2;
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn gen_octagonal(lower: i64, upper: i64) -> FxHashSet<i64> {
    let mut result = FxHashSet::default();
    let mut m = 1;
    loop {
        let n = m * (3 * m - 2);
        if n >= upper {
            break;
        }

        if lower <=n && n < upper {
            result.insert(n);
        }

        m += 1;
    }

    result
}

fn find_position(n: i64, num_sets: &[FxHashSet<i64>]) -> u64 {
    let mut result = 0;

    for (i, num_set)in num_sets.iter().enumerate() {
        if num_set.contains(&n) {
            result |= 1 << i;
        }
    }

    result
}

fn find_position_list(nums: &[i64], num_sets: &[FxHashSet<i64>]) -> u64 {
    let mut result = 0;

    for n in nums {
        result |= find_position(*n, num_sets);
    }

    result
}

fn find_cyclic_candidates_recursive(nums: &[i64], num_sets: &[FxHashSet<i64>], result: &mut Vec<Vec<i64>>, state: &mut [i64], index: usize) {
    if index == num_sets.len() {
        if state[0] / 100 == state[index - 1] % 100 {
            result.push(state.to_vec());
        }

    } else {
        let prev_position = find_position_list(state, num_sets);
        for n in nums {
            state[index] = *n;

            if index == 0 {
                find_cyclic_candidates_recursive(nums, num_sets, result, state, index + 1);

            } else {
                if state[index - 1] % 100 == state[index] / 100 {
                    let n_position = find_position(*n, num_sets);
                    if n_position | prev_position > prev_position {
                        find_cyclic_candidates_recursive(nums, num_sets, result, state, index + 1);
                    }
                }
            }

            state[index] = 0;
        }
    }
}

fn find_cyclic_candidates(nums: &[i64], num_sets: &[FxHashSet<i64>]) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    let mut state = vec![0; num_sets.len()];

    find_cyclic_candidates_recursive(nums, num_sets, &mut result, &mut state, 0);

    result
}


const LOWER_LIMIT: i64 = 1000;
const UPPER_LIMIT: i64 = 10000;

pub fn solve() -> i64 {
    let triangle = gen_triangle(LOWER_LIMIT, UPPER_LIMIT);
    let square = gen_square(LOWER_LIMIT, UPPER_LIMIT);
    let pentagonal = gen_pentagonal(LOWER_LIMIT, UPPER_LIMIT);
    let hexagonal = gen_hexagonal(LOWER_LIMIT, UPPER_LIMIT);
    let heptagonal = gen_heptagonal(LOWER_LIMIT, UPPER_LIMIT);
    let octagonal = gen_octagonal(LOWER_LIMIT, UPPER_LIMIT);

    // println!("triangle:     {:?}", triangle);
    // println!("square:       {:?}", square);
    // println!("pentagonal:   {:?}", pentagonal);
    // println!("hexagonal:    {:?}", hexagonal);
    // println!("heptagonal:   {:?}", heptagonal);
    // println!("octagonal:    {:?}", octagonal);

    let num_lists = vec![triangle, square, pentagonal, hexagonal, heptagonal, octagonal];
    let mut nums_set = FxHashSet::default();
    for list in &num_lists {
        for &n in list {
            nums_set.insert(n);
        }
    }
    let mut nums = nums_set.iter().cloned().collect::<Vec<i64>>();
    nums.sort();

    // println!("nums: {:?}", nums);
    let result = find_cyclic_candidates(&nums, &num_lists);
    if let Some(r) = result.into_iter().next() {
        let mut sum = 0;
        for n in r {
            sum += n;
        }

        return sum;
    }

    0
}
