use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(54, "Poker Hands")
        .with_answer(376)
        .solution("naive", naive::solve)
);

pub fn load() -> Vec<Vec<String>> {
    let raw = Problem::load_data();
    let content = String::from_utf8(raw).unwrap();

    content.lines()
        .map(|line|
            line.split(' ')
            .map(String::from)
            .collect())
        .collect()
}
