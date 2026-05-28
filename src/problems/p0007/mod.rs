use crate::common::Problem;

mod naive;
mod sieve_full;
mod sieve_half;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(7, "10001st Prime")
        .with_answer(104743)
        .solution("naive", naive::solve)
        .solution("sieve (full size)", sieve_full::solve)
        .solution("sieve (half size)", sieve_half::solve)
);
