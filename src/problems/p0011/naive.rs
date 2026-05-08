use super::matrix::GRID;

pub fn solve() -> i64 {
    let mut result = 0;

    // false positive for this loop
    #[allow(clippy::needless_range_loop)]
    for x in 0..20 {
        for y in 0..17 {
            let r1 = GRID[x][y] * GRID[x][y + 1] * GRID[x][y + 2] * GRID[x][y + 3];
            let r2 = GRID[y][x] * GRID[y + 1][x] * GRID[y + 2][x] * GRID[y + 3][x];
            result = result.max(r1).max(r2);
        }
    }

    for x in 0..17 {
        for y in 0..17 {
            let r1 = GRID[x][y] * GRID[x + 1][y + 1] * GRID[x + 2][y + 2] * GRID[x + 3][y + 3];
            let r2 = GRID[x][y + 3] * GRID[x + 1][y + 2] * GRID[x + 2][y + 1] * GRID[x + 3][y];
            result = result.max(r1).max(r2);
        }
    }

    result
}
