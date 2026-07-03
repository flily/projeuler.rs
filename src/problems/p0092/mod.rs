use crate::framework::Problem;

mod common;

mod naive;
mod cache_hashset;
mod cache_fxhashset;
// mod cache_ahash;     // enable and run cargo add ahash
// mod cache_rapidhash; // enable and run cargo add rapidhash
mod cache_vector;
mod cache_vectorlet;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(92, "Square digit chains")
        .with_answer(8581146)
        .solution("naive", naive::solve)
        .solution("with cache (HashSet)", cache_hashset::solve)
        .solution("with cache (FxHashSet)", cache_fxhashset::solve)
        // .solution("with cache (AHash)", cache_ahash::solve)
        // .solution("with cache (RapidHash)", cache_rapidhash::solve)
        .solution("with cache (Vector)", cache_vector::solve)
        .solution("with cache (reduced Vector)", cache_vectorlet::solve)
);
