use crate::common::Problem;

mod naive;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(46, "Goldbach's Other Conjecture")
        .with_answer(5777)
        .solution("naive", naive::solve)
);
