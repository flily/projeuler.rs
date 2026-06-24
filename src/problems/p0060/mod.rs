use crate::common::Problem;

mod naive;
mod optimized;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(60, "Prime Pair Sets")
        .with_answer(26033)
        .solution("naive", naive::solve)
        .solution("search pair list", optimized::solve)
);
