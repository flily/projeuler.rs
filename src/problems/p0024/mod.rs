use crate::framework::Problem;

mod naive;
mod math;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(24, "Lexicographic Permutations")
        .with_answer(2783915460)
        .solution("naive", naive::solve)
        .solution("math", math::solve)
);
