use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(2, "Even Fibonacci Numbers")
        .with_answer(4613732)
        .solution("naive", naive::solve)
);
