fn update_cell(matrix: &Vec<Vec<i64>>,  path: &mut Vec<Vec<Option<i64>>>, position: (usize, usize)) {
    let (row, col) = position;
    let height = matrix.len();

    let current = matrix[row][col];
    let mut neighbours = Vec::with_capacity(4);

    if col == 0 {
        neighbours.push(current);
    }

    if row > 0 {
        if let Some(upper) = path[row - 1][col] {
            neighbours.push(current + upper);
        }
    }

    if row < height - 1 {
        if let Some(lower) = path[row + 1][col] {
            neighbours.push(current + lower);
        }
    }

    if col > 0 {
        if let Some(left) = path[row][col - 1] {
            neighbours.push(current + left);
        }
    }

    path[row][col] = neighbours.into_iter().min();
}

fn search(matrix: &Vec<Vec<i64>>, size: (usize, usize)) -> i64 {
    let (width, height) = size;

    let mut path = vec![vec![None; width]; height];
    for row in 0..height {
        path[row][0] = Some(matrix[row][0]);
    }

    for y in 0..width {
        for x in 0..height {
            update_cell(matrix, &mut path, (x, y));
        }

        for x in (0..height).rev() {
            update_cell(matrix, &mut path, (x, y));
        }
    }

    path.iter()
        .map(|row| row[width - 1].unwrap())
        .min().unwrap()
}

pub fn solve() -> i64 {
    let matrix = super::load();
    let size = (matrix[0].len(), matrix.len());
    search(&matrix, size)
}
