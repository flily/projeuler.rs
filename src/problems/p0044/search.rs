use rustc_hash::FxHashSet;

fn pentagonal(n: i64) -> i64 {
    n * (3 * n - 1) / 2
}

pub fn solve_less_hashset() -> i64 {
    let max_n = 3000_usize;
    let p_list = (0..max_n).map(|n| pentagonal(n as i64)).collect::<Vec<i64>>();
    let p_set = p_list.iter().copied().collect::<FxHashSet<i64>>();

    let mut min_d = p_list[max_n - 1];
    let mut found = false;

    for j in 1..max_n {
        let pj = p_list[j];
        for pk in p_list.iter().take(j-1).skip(1) {
            let d = pj - pk;
            if p_set.contains(&(pj + pk)) && p_set.contains(&d) && d < min_d {
                min_d = d;
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    min_d
}

pub fn solve_larger_hashset() -> i64 {
    let max_n = 3000_usize;
    let p_list = (0..max_n).map(|n| pentagonal(n as i64)).collect::<Vec<i64>>();
    let p_set = p_list.iter().copied().collect::<FxHashSet<i64>>();

    let mut min_d = p_list[max_n - 1];
    let mut found = false;

    for j in 1..max_n {
        let pj = p_list[j];
        for pk in p_list.iter().skip(j+1) {
            let d = pk - pj;
            if p_set.contains(&(pj + pk)) && p_set.contains(&d) && d < min_d {
                min_d = d;
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    min_d
}

pub fn solve_less_bsearch() -> i64 {
    let max_n = 3000_usize;
    let p_list = (0..max_n).map(|n| pentagonal(n as i64)).collect::<Vec<i64>>();

    let mut min_d = p_list[max_n - 1];
    let mut found = false;

    for j in 1..max_n {
        let pj = p_list[j];
        for k in 1..(j-1) {
            let pk = p_list[k];
            let d = pj - pk;
            if p_list.binary_search(&(pj + pk)).is_ok() && p_list.binary_search(&d).is_ok() && d < min_d {
                min_d = d;
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    min_d
}

pub fn solve_larger_bsearch() -> i64 {
    let max_n = 3000_usize;
    let p_list = (0..max_n).map(|n| pentagonal(n as i64)).collect::<Vec<i64>>();

    let mut min_d = p_list[max_n - 1];
    let mut found = false;

    for j in 1..max_n {
        let pj = p_list[j];
        for k in (j+1)..max_n {
            let pk = p_list[k];
            let d = pk - pj;
            if p_list.binary_search(&(pj + pk)).is_ok() && p_list.binary_search(&d).is_ok() && d < min_d {
                min_d = d;
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    min_d
}
