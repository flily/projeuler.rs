use crate::common::{Problem, SolutionInfo};

mod string;
mod integer;
mod algebraic;
mod generate;
mod order;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {
    id: 4,
    title: "Largest Palindrome Product",
    answer: 906609,
    extra_time_ms: 0,
    solutions: vec![
        SolutionInfo {
            name: "by string (forward search)",
            entry: string::solve,
        },
        SolutionInfo {
            name: "by string (reverse vector)",
            entry: string::solve_reverse_vec,
        },
        SolutionInfo {
            name: "by string (reverse loop)",
            entry: string::solve_reverse_loop,
        },
        SolutionInfo {
            name: "by integer (forward search)",
            entry: integer::solve,
        },
        SolutionInfo {
            name: "by integer (reverse vector)",
            entry: integer::solve_reverse_vec,
        },
        SolutionInfo {
            name: "by integer (reverse loop)",
            entry: integer::solve_reverse_loop,
        },
        SolutionInfo {
            name: "by integer algebraic (forward search)",
            entry: algebraic::solve,
        },
        SolutionInfo {
            name: "by integer algebraic (reverse vector)",
            entry: algebraic::solve_reverse_vec,
        },
        SolutionInfo {
            name: "by integer algebraic (reverse loop)",
            entry: algebraic::solve_reverse_loop,
        },
        SolutionInfo {
            name: "by generating palindrome",
            entry: generate::solve,
        },
        SolutionInfo {
            name: "by multiplying orders iterate",
            entry: order::solve,
        },
    ],
});
