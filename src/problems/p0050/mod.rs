use crate::framework::Problem;

mod naive;
mod compare_sum;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(50, "Consecutive Prime Sum")
        .with_answer(997651)
        .solution("naive", naive::solve)
        .solution("compare_sum", compare_sum::solve)
);
