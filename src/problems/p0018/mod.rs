use crate::common::Problem;

mod triangle;

mod bruteforce;
mod flood;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(18, "Maximum Path Sum I")
        .with_answer(1074)
        .solution("bruteforce (recursive)", bruteforce::solve)
        .solution("flood fill", flood::solve)
);
