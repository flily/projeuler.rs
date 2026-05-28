use crate::common::Problem;

mod naive;
mod factor_filter;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(12, "Highly Divisible Triangular Number")
        .with_answer(76576500)
        .solution("naive", naive::solve)
        .solution("factor filter (3 primes)", factor_filter::solve_3)
        .solution("factor filter (6 primes)", factor_filter::solve_6)
        .solution("factor filter (7 primes)", factor_filter::solve_7)
);
