use crate::framework::Problem;

mod string;
mod integer;
mod algebraic;
mod generate;
mod order;

pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(||
    Problem::init(4, "Largest Palindrome Product")
        .with_answer(906609)
        .solution("by string (forward search)", string::solve)
        .solution("by string (reverse vector)", string::solve_reverse_vec)
        .solution("by string (reverse loop)", string::solve_reverse_loop)
        .solution("by integer (forward search)", integer::solve)
        .solution("by integer (reverse vector)", integer::solve_reverse_vec)
        .solution("by integer (reverse loop)", integer::solve_reverse_loop)
        .solution("by integer algebraic (forward search)", algebraic::solve)
        .solution("by integer algebraic (reverse vector)", algebraic::solve_reverse_vec)
        .solution("by integer algebraic (reverse loop)", algebraic::solve_reverse_loop)
        .solution("by generating palindrome", generate::solve)
        .solution("by multiplying orders iterate", order::solve)
);
