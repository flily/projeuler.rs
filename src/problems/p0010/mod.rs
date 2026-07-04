use crate::framework::Problem;

mod naive;
mod sieve;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(10, "Summation of Primes")
        .with_answer(142913828922)
        .solution("naive", naive::solve)
        .solution("sieve", sieve::solve)
);
