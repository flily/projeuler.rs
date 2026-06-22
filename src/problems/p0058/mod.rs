use crate::common::Problem;

mod naive;
mod skip_bottom_right;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(58, "Spiral Primes")
        .with_answer(26241)
        .solution("naive", naive::solve)
        .solution("skip bottom right", skip_bottom_right::solve)
);
