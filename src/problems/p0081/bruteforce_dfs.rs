fn search(matrix: &Vec<Vec<i64>>, size: (usize, usize), position: (usize, usize), sum: i64) -> i64 {
    let (width, height) = size;
    let (row, col) = position;

    let current = matrix[row][col];
    let next_sum = sum + current;

    let result_right = if col < width - 1 {
        search(matrix, size, (row, col + 1), next_sum)
    } else {
        -1
    };

    let result_down = if row < height - 1 {
        search(matrix, size, (row + 1, col), next_sum)
    } else {
        -1
    };

    if result_right < 0 && result_down < 0 {
        next_sum

    } else if result_right < 0 {
        result_down

    } else if result_down < 0 {
        result_right

    } else {
        result_right.min(result_down)
    }
}

pub fn solve() -> i64 {
    let matrix = super::load();
    let size = (matrix[0].len(), matrix.len());
    search(&matrix, size, (0, 0), 0)
}
