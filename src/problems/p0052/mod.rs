use crate::framework::Problem;

mod hashset;
mod table;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(52, "Permuted Multiples")
        .with_answer(142857)
        .solution("use hashset (FxHashSet)", hashset::solve)
        .solution("use table", table::solve)
);
