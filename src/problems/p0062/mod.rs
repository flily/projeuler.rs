use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(62, "Cubic permutations")
        .with_answer(127035954683)
        .solution("naive", naive::solve)
);
