use crate::framework::Problem;

mod bfs_flood;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(82, "Path Sum: Three Ways")
        .with_answer(260324)
        .solution("breadth first, flood", bfs_flood::solve)
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
