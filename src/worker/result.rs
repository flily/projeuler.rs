use std::time;

use colored::{Color, Colorize};

use super::message;

#[derive(Debug, Clone, PartialEq, Eq)]
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
            Self::None => "-",
            Self::Unknown => "?",
            Self::Correct => "correct",
            Self::Wrong => "wrong",
            Self::Timeout => "timeout",
            Self::Crash => "crash",
        }
    }

    pub fn color(&self) -> colored::Color {
        match self {
            Self::Correct => Color::Green,
            Self::Unknown => Color::Yellow,
            Self::Wrong => Color::Red,
            Self::Timeout => Color::Yellow,
            Self::Crash => Color::Red,
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
    pub fn init(solution: String, answer: Option<i64>, extra_timeout_ms: u64) -> Self {
        Self {
            solution,
            answer,
            got: None,
            result: FinalResult::None,
            cost: time::Duration::from_secs(0),
            extra_timeout_ms,
        }
    }

    pub fn finish(mut self, got: i64, cost: time::Duration) -> Self {
        self.got = Some(got);
        self.cost = cost;
        self.result = FinalResult::Unknown;
        self
    }

    pub fn timeout(mut self, cost: time::Duration) -> Self {
        self.result = FinalResult::Timeout;
        self.cost = cost;
        self
    }

    pub fn crash(mut self, cost: time::Duration) -> Self {
        self.result = FinalResult::Crash;
        self.cost = cost;
        self
    }

    pub fn with_check(mut self, check: bool) -> Self {
        if check {
            self.check();
        }
        self
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
