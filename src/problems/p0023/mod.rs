use crate::common::Problem;

mod naive;
mod cache;
mod substraction;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(23, "Non-Abundant Sums")
        .with_answer(4179871)
        .solution("naive", naive::solve)
        .solution("cache", cache::solve)
        .solution("substraction", substraction::solve)
);
