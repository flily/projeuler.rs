use crate::framework::Problem;

mod naive;
mod facttable;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(34, "Digit Factorials")
        .with_answer(40730)
        .solution("naive", naive::solve)
        .solution("factorial table", facttable::solve)
);
