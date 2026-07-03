use crate::framework::Problem;

mod permutation;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(43, "Sub-string Divisibility")
        .with_answer(16695334890)
        .solution("permutation", permutation::solve)
);
