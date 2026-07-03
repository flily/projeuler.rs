use crate::framework::Problem;

mod naive;
mod cache_hashmap;
mod cache_fxhashmap;
mod cache_vector;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(14, "Longest Collatz sequence")
        .with_answer(837799)
        .solution("naive", naive::solve)
        .solution("with cache (HashMap)", cache_hashmap::solve)
        .solution("with cache (FxHashMap)", cache_fxhashmap::solve)
        .solution("with cache (Vector)", cache_vector::solve)
);
