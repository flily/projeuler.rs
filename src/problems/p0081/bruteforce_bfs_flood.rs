fn update_cell(matrix: &[Vec<i64>], path: &mut [Vec<i64>], position: (usize, usize)) {
    let (row, col) = position;

    let current = matrix[row][col];

    let upper = if row > 0 {
        Some(path[row - 1][col])
    } else {
        None
    };

    let left = if col > 0 {
        Some(path[row][col - 1])
    } else {
        None
    };

    path[row][col] = current + match (upper, left) {
        (Some(u), Some(l)) => u.min(l),
        (Some(v), None) => v,
        (None, Some(v)) => v,
        (None, None) => 0,
    }
}

fn search(matrix: &[Vec<i64>], size: (usize, usize)) -> i64 {
    let (width, height) = size;

    let mut path = vec![vec![0; width]; height];
    path[0][0] = matrix[0][0];

    for i in 0..(width + height) {
        for x in 0..=i {
            let y = i - x;
            if x < width && y < height {
                update_cell(matrix, &mut path, (y, x));
            }
        }
    }

    path[height - 1][width - 1]
}

pub fn solve() -> i64 {
    let matrix = super::load();
    let size = (matrix[0].len(), matrix.len());
    search(&matrix, size)
}
