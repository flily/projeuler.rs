use crate::common::Problem;

mod naive;
mod remove_factor;
mod prime_list;
mod count;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(47, "Distinct Primes Factors")
        .with_answer(134043)
        .solution("naive", naive::solve)
        .solution("remove factor", remove_factor::solve)
        .solution("count", count::solve)
        .solution("prime list", prime_list::solve)
);
