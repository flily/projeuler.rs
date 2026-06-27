use crate::common::Problem;

mod naive;
mod limit;
mod sieve;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(69, "Totient Maximum")
        .with_answer(510510)
        .solution("naive", naive::solve)
        .solution("nphi limit", limit::solve)
        .solution("sieve", sieve::solve)
);
