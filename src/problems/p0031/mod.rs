use crate::common::Problem;

mod bruteforce;
mod branchcut;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(31, "Coin Sums")
        .with_answer(73682)
        .solution("naive", bruteforce::solve)
        .solution("branch cut", branchcut::solve)
);
