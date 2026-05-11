use super::load;

fn max_path_sum(triangle: &Vec<Vec<i64>>, row: usize, col: usize, sum: i64) -> i64 {
    let current = triangle[row][col];
    let current_sum = sum + current;
    if row >= triangle.len() - 1 {
        return current_sum;
    }

    let left = max_path_sum(triangle, row + 1, col, current_sum);
    let right = max_path_sum(triangle, row + 1, col + 1, current_sum);
    left.max(right)
}

pub fn solve() -> i64 {
    let triangle = load();
    max_path_sum(&triangle, 0, 0, 0)
}
