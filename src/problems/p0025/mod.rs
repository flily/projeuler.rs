use crate::framework::Problem;

mod naive;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(25, "1000-digit Fibonacci number")
        .with_answer(4782)
        .solution("naive BigInt (num-bigint)", naive::solve)
        .solution("naive BigInt no array (num-bigint)", naive::solve_no_array)
        .solution("BigInt (malachite)", tpmalachite::solve)
);
