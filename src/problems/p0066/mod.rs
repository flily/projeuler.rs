use crate::common::Problem;

mod naive;
mod formula;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(66, "Diophantine Equation")
        .with_answer(661)
        .solution("naive", naive::solve)
        .solution("Pell's Equation formula", formula::solve)
);
