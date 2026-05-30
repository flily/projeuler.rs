use crate::common::Problem;

mod naive;
mod remove_factor;
mod prime_table;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(3, "Problem 3")
        .with_answer(6857)
        .solution("naive", naive::solve)
        .solution("remove_factor", remove_factor::solve)
        .solution("prime_table", prime_table::solve)
);
