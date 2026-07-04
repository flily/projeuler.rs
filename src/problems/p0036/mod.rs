use crate::framework::Problem;

mod naive;
mod generator;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(36, "Double-base Palindromes")
        .with_answer(872187)
        .solution("naive", naive::solve)
        .solution("10-palindromes generator", generator::solve)
);
