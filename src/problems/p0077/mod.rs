use crate::framework::Problem;

mod bruteforce;
mod bruteforce_cache;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(77, "Prime Summations")
        .with_answer(71)
        .solution("bruteforce", bruteforce::solve)
        .solution("bruteforce (with cache)", bruteforce_cache::solve)
);
