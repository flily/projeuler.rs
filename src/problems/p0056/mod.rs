use crate::common::Problem;

mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(56, "Powerful Digit Counts")
        .with_answer(972)
        .solution("with BigInt (num-bigint)", tpnumbigint::solve)
        .solution("with BigInt (malachite)", tpmalachite::solve)
);
