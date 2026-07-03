use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(64, "Odd Period Square Roots")
        .with_answer(1322)
        .solution("naive", naive::solve)
);
