use crate::framework::Problem;

mod bruteforce_dfs;
mod bruteforce_bfs_flood;
mod bruteforce_bfs_mark;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(81, "Path Sum: Two Ways")
        .with_answer(427337)
        .solution("brute force, depth first search", bruteforce_dfs::solve)
        .solution("brute force, breadth first, flood", bruteforce_bfs_flood::solve)
        .solution("brute force, breadth first, mark", bruteforce_bfs_mark::solve)
);

pub fn load() -> Vec<Vec<i64>> {
    let raw = Problem::load_data();
    let content = String::from_utf8(raw).unwrap();
    content.lines()
        .map(|line|
            line.split(",").map(|item|
                item.parse().unwrap()
            ).collect()
        ).collect()
}