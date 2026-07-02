use crate::common::Problem;

mod naive;
mod sieve;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(72, "Counting Fractions")
        .with_answer(303963552391)
        .solution("naive", naive::solve)
        .solution("quick", sieve::solve)
);
