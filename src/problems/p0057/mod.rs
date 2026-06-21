use crate::common::Problem;

mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(57, "Square Root Convergents")
        .with_answer(153)
        .solution("naive BigInt (num-bigint)", tpnumbigint::solve)
        .solution("naive BigInt (malachite)", tpmalachite::solve)
        .solution("generator with BigInt (num-bigint)", tpnumbigint::solve_generator)
        .solution("generator with BigInt (malachite)", tpmalachite::solve_generator)
        .solution("directly with BigInt (num-bigint)", tpnumbigint::solve_directly)
        .solution("directly with BigInt (malachite)", tpmalachite::solve_directly)
);
