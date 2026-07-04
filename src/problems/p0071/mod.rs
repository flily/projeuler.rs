use crate::framework::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(71, "Ordered Fractions")
        .with_answer(428570)
        .solution("naive", naive::solve)
);
