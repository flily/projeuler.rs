use std::fs;
use std::time;
use std::panic::Location;
use std::path::Path;

use crate::worker::{FinalResult, RunResult};

pub type Solution = fn() -> i64;

static DATA_PATH_BASE: &str = "data";

#[derive(Debug, Clone)]
pub struct SolutionInfo {
    pub name: &'static str,
    pub entry: Solution,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: i64,
    pub title: &'static str,
    pub answer: i64,
    pub extra_time_ms: u64,
    pub solutions: Vec<SolutionInfo>,
}

impl Problem {
    pub fn from_id(id: i64) -> Self {
        Problem {
            id,
            title: "",
            answer: 0,
            extra_time_ms: 0,
            solutions: vec![],
        }
    }

    pub fn get_solution(&self, index: usize) -> Option<SolutionInfo> {
        if index >= self.solutions.len() {
            None
        } else {
            Some(self.solutions[index].clone())
        }
    }

    pub fn make_run_result_for(&self, index: usize) -> Option<RunResult> {
        self.get_solution(index).map(|sln| RunResult {
                solution: sln.name.to_string(),
                answer: Some(self.answer),
                got: None,
                result: FinalResult::None,
                cost: time::Duration::from_secs(0),
                extra_timeout_ms: self.extra_time_ms,
        })
    }

    pub fn make_solution_items(&self) -> Vec<SolutionItem> {
        self.solutions.iter().enumerate().map(|(index, sln)| SolutionItem {
            id: self.id,
            index: index as i64,
            problem_title: self.title.to_string(),
            answer: Some(self.answer),
            extra_time_ms: self.extra_time_ms,
            solution_name: sln.name.to_string(),
            entry: sln.entry,
         }).collect()
    }
}

pub struct SolutionItem {
    pub id: i64,
    pub index: i64,
    pub problem_title: String,
    pub answer: Option<i64>,
    pub extra_time_ms: u64,
    pub solution_name: String,
    pub entry: Solution,
}

impl SolutionItem {
    pub fn run_result(&self) -> RunResult {
        RunResult {
            solution: self.solution_name.to_string(),
            answer: self.answer,
            got: None,
            result: FinalResult::None,
            cost: time::Duration::from_secs(0),
            extra_timeout_ms: self.extra_time_ms,
        }
    }

    pub fn finish_result(&self, got: i64, cost: time::Duration) -> RunResult {
        RunResult {
            solution: self.solution_name.to_string(),
            answer: self.answer,
            got: Some(got),
            result: FinalResult::Unknown,
            cost,
            extra_timeout_ms: self.extra_time_ms,
        }
    }

    pub fn timeout_result(&self, cost: time::Duration) -> RunResult {
        RunResult {
            solution: self.solution_name.to_string(),
            answer: self.answer,
            got: None,
            result: FinalResult::Timeout,
            cost,
            extra_timeout_ms: self.extra_time_ms,
        }
    }

    pub fn crash_result(&self, cost: time::Duration) -> RunResult {
        RunResult {
            solution: self.solution_name.to_string(),
            answer: self.answer,
            got: None,
            result: FinalResult::Crash,
            cost,
            extra_timeout_ms: self.extra_time_ms,
        }
    }
}

#[track_caller]
pub fn load_data() -> Vec<u8> {
    let caller = Location::caller();
    let caller_path = Path::new(caller.file());

    let parent = match caller_path.parent() {
        Some(p) => p,
        None => {
            panic!("call load_data() in a solution of problem, in directory src/problems/pXXXX");
        }
    };
    let module_name = match parent.file_name() {
        Some(p) => p.to_str().unwrap(),
        None => {
            panic!("call load_data() in a solution of problem, in directory src/problems/pXXXX");
        }
    };

    let data_path_base = Path::new(DATA_PATH_BASE);
    if !data_path_base.exists() {
        panic!("data directory not found: {}", DATA_PATH_BASE);
    }

    for entry in fs::read_dir(data_path_base).unwrap().flatten() {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name.starts_with(module_name) {
            let content = fs::read(path);
            match content {
                Ok(content) => {
                    return content;
                }
                Err(e) => {
                    panic!(
                        "Failed to read data file for problem {}: {}",
                        module_name, e,
                    );
                }
            }
        }
    }

    panic!("no data file found for problem {}", module_name);
}
