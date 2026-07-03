use crate::framework::Problem;

mod naive;
mod sort;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(79, "Passcode Derivation")
        .with_answer(73162890)
        .solution("naive", naive::solve)
        .solution("sort", sort::solve)
);

pub fn load() -> Vec<i64> {
    let raw = Problem::load_data();
    let content = String::from_utf8(raw).unwrap();
    content.lines().map(|line| line.parse().unwrap()).collect()
}
