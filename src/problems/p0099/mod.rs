use crate::framework::Problem;

mod naive;
mod log;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(99, "Largest Exponential")
        .with_answer(709)
        .solution("naive", naive::solve)
        .solution("logarithm", log::solve)
);

pub fn load() -> Vec<(i64, i64)> {
    let raw = Problem::load_data();
    let content = String::from_utf8(raw).unwrap();
    content.lines()
        .map(|line| {
            let nums = line.split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();
            (nums[0], nums[1])
        })
        .collect()
}
