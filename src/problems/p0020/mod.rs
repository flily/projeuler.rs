use crate::framework::Problem;

mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(20, "Factorial Digit Sum")
        .with_answer(648)
        .solution("BigInt (num-bigint) to_string()", tpnumbigint::solve_string)
        .solution("BigInt (num-bigint) by math", tpnumbigint::solve_math)
        .solution("BigInt (malachite)", tpmalachite::solve_string)
);
