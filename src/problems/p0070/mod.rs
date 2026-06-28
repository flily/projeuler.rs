use crate::common::Problem;

mod naive;
mod sieve;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(70, "Totient Permutation")
        .with_answer(8319823)
        .solution("naive", naive::solve)
        .solution("sieve", sieve::solve)
);
