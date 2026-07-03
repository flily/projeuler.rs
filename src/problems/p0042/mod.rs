use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(42, "Coded Triangle Numbers")
        .with_answer(162)
        .solution("naive", naive::solve)
);

pub fn load() -> Vec<String> {
    let raw = Problem::load_data();
    raw.split(|&c| c == b',')
        .map(|s| s[1..s.len() - 1].iter().map(|&c| c as char).collect())
        .collect()
}
