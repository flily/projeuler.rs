use std::time;

use colored::{Color, Colorize};

use crate::common::Solution;

use super::message;

#[derive(Debug, Clone)]
pub enum FinalResult {
    None,       // Not run yet
    Unknown,    // Run but not checked
    Correct,
    Wrong,
    Timeout,
    Crash,
}

impl FinalResult {
    pub fn to_string(&self) -> &str {
        match self {
            FinalResult::None => "-",
            FinalResult::Unknown => "?",
            FinalResult::Correct => "correct",
            FinalResult::Wrong => "wrong",
            FinalResult::Timeout => "timeout",
            FinalResult::Crash => "crash",
        }
    }

    pub fn color(&self) -> colored::Color {
        match self {
            FinalResult::Correct => Color::Green,
            FinalResult::Unknown => Color::Yellow,
            FinalResult::Wrong => Color::Red,
            FinalResult::Timeout => Color::Yellow,
            FinalResult::Crash => Color::Red,
            _ => Color::White,
        }
    }

    pub fn color_string(&self) -> colored::ColoredString {
        self.to_string().color(self.color())
    }

    pub fn color_on(&self, s: &str) -> colored::ColoredString {
        s.color(self.color())
    }
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub solution: String,
    pub answer: Option<i64>,
    pub got: Option<i64>,
    pub result: FinalResult,
    pub cost: time::Duration,
    pub extra_timeout_ms: u64,
}

impl RunResult {
    pub fn basic(answer: i64, cost: time::Duration) -> Self {
        Self {
            solution: String::new(),
            answer: None,
            got: Some(answer),
            result: FinalResult::Unknown,
            cost,
            extra_timeout_ms: 0,
        }
    }

    pub fn timeout(cost: time::Duration) -> Self {
        Self {
            solution: String::new(),
            answer: None,
            got: None,
            result: FinalResult::Timeout,
            cost,
            extra_timeout_ms: 0,
        }
    }

    pub fn check(&mut self) -> FinalResult {
        self.result = match (self.got, self.answer) {
            (Some(got), Some(ans)) => {
                if got == ans {
                    FinalResult::Correct
                } else {
                    FinalResult::Wrong
                }
            },
            _ => self.result.clone(),
        };

        self.result.clone()
    }
}

#[derive(Debug)]
pub enum RunError {
    ProblemNotFound {
        problem_id: i64,
    },
    SolutionNotFound {
        problem_id: i64,
        solution_id: usize,
    },
    NetworkError {
        source: std::io::Error,
    },
    ProtocolMessageError {
        source: message::MessageError,
    },
    Timeout,
}
