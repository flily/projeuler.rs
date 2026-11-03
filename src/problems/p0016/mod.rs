use crate::framework::Problem;

mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(16, "Power Digit Sum")
        .with_answer(1366)
        .solution("BigInt as integer (num-bigint)", tpnumbigint::solve_integer)
        .solution("BigInt as string (num-bigint)", tpnumbigint::solve_string)
        .solution("BigInt as string (malachite)", tpmalachite::solve)
);
