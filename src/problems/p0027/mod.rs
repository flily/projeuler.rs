use crate::framework::Problem;

mod naive;
mod cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(27, "Quadratic Primes")
        .with_answer(-59231)
        .solution("naive", naive::solve)
        .solution("cache", cache::solve)
);
