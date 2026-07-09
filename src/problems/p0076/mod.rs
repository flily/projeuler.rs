use crate::framework::Problem;

mod bruteforce;
mod bruteforce_cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(76, "Counting Summations")
        .with_answer(190569291)
        .solution("bruteforce", bruteforce::solve)
        .solution("bruteforce (with cache)", bruteforce_cache::solve)
);
