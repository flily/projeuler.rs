use std::collections::VecDeque;

fn update_cell(matrix: &Vec<Vec<i64>>, path: &mut Vec<Vec<i64>>, position: (usize, usize)) {
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

fn search(matrix: &Vec<Vec<i64>>, size: (usize, usize)) -> i64 {
    let (width, height) = size;

    let mut path = vec![vec![0; width]; height];
    path[0][0] = matrix[0][0];

    let mut queue = VecDeque::new();
    queue.push_back((0_usize, 0_usize));

    let mut stack_count = vec![vec![0; width]; height];
    stack_count[0][0] = 1;

    loop {
        let (x, y) = match queue.pop_front() {
            Some(pos) => pos,
            None => break,
        };

        update_cell(matrix, &mut path, (y, x));
        stack_count[x][y] -= 1;

        if x + 1 < width {
            let c = stack_count[x + 1][y];
            if c <= 0 {
                queue.push_back((x + 1, y));
                stack_count[x + 1][y] = c + 1;
            }
        }

        if y + 1 < height {
            let c = stack_count[x][y + 1];
            if c <= 0 {
                queue.push_back((x, y + 1));
                stack_count[x][y + 1] = c + 1;
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
