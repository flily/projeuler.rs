use crate::common::Problem;

mod naive;
mod generator;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(37, "Truncatable Primes")
        .with_answer(748317)
        .solution("naive", naive::solve)
        .solution("generator", generator::solve)
);
