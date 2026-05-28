use crate::common::Problem;

mod naive;
mod factorial_table;
mod cachefinal;
mod cachechain1;
mod cachechain2;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(74, "Digit Factorial Chains")
        .with_answer(402)
        .solution("naive", naive::solve)
        .solution("factorial table", factorial_table::solve)
        .solution("cache final results", cachefinal::solve)
        .solution("cache chain (with HashSet index)", cachechain1::solve)
        .solution("cache chain (no HashSet index)", cachechain2::solve)
);
