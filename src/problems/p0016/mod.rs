use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(16, "Power Digit Sum")
        .with_answer(1366)
        .solution("naive", naive::solve)
);
