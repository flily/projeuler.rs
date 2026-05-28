use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(26, "Reciprocal Cycles")
        .with_answer(983)
        .solution("naive", naive::solve)
        .solution("on prime denominators", naive::solve_prime)
);
