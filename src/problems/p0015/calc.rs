fn calc_lattice_path(width: i64, height: i64) -> i64 {
    let mut row = vec![1; (width + 1) as usize];
    for _ in 0..height {
        for i in 1..=width {
            row[i as usize] += row[i as usize - 1];
        }
    }

    row[width as usize]
}

pub fn solve() -> i64 {
    calc_lattice_path(20, 20)
}
