use super::load;

fn max_path_sum(triangle: &[Vec<i64>]) -> i64 {
    let mut result = triangle[0].clone();
    for (x, row) in triangle.iter().enumerate().skip(1) {
        let mut new_row = Vec::new();
        for (i, v) in row.iter().enumerate() {
            if *v == 0 {
                break;
            }

            let left = if i > 0 { result[i - 1] } else { 0 };
            let right = if i < x { result[i] } else { 0 };
            new_row.push(left.max(right) + v);
        }
        result = new_row;
    }

    result.into_iter().max().unwrap()
}

pub fn solve() -> i64 {
    let triangle = load();
    max_path_sum(&triangle)
}
