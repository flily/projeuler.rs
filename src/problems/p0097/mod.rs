use crate::common::{Problem, SolutionInfo};

mod modulo;
mod tpnumbigint;
mod tpmalachite;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 97,
    title: "Large Non-Mersenne Prime",
    answer: 8739992577,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "multiply with modulo",
            entry: modulo::solve,
        },
        SolutionInfo {
            name: "naive BigInt (num-bigint)",
            entry: tpnumbigint::solve,
        },
        SolutionInfo {
            name: "naive BigInt (malachite)",
            entry: tpmalachite::solve,
        },
    ],
});
