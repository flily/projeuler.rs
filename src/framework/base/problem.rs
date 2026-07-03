use std::fs;
use std::time;
use std::panic::Location;
use std::path::Path;

use crate::framework::RunResult;

pub type Solution = fn() -> i64;

static DATA_PATH_BASE: &str = "data";

#[derive(Debug, Clone)]
pub struct SolutionInfo {
    pub name: String,
    pub entry: Solution,
}

impl SolutionInfo {
    pub fn new(name: &str, entry: Solution) -> Self {
        SolutionInfo {
            name: name.to_string(),
            entry,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: i64,
    pub title: String,
    pub answer: Option<i64>,
    pub extra_time_ms: u64,
    pub solutions: Vec<SolutionInfo>,
}

impl Problem {
    pub fn init(id: i64, title: &str) -> Self {
        Problem {
            id,
            title: title.to_string(),
            answer: None,
            extra_time_ms: 0,
            solutions: vec![],
        }
    }

    pub fn from_id(id: i64) -> Self {
        Self::init(id, "")
    }

    pub fn with_answer(mut self, answer: i64) -> Self {
        self.answer = Some(answer);
        self
    }

    pub fn extra_timeout_ms(mut self, ms: u64) -> Self {
        self.extra_time_ms = ms;
        self
    }

    pub fn solution(mut self, name: &str, entry: Solution) -> Self {
        self.solutions.push(SolutionInfo {
            name: name.to_string(),
            entry,
        });
        self
    }

    pub fn with_solutions(mut self, solutions: Vec<SolutionInfo>) -> Self {
        self.solutions.extend(solutions);
        self
    }

    pub fn get_solution(&self, index: usize) -> Option<SolutionItem> {
        self.solutions.get(index).map(|sln| SolutionItem {
            id: self.id,
            index: index as i64,
            answer: self.answer,
            extra_time_ms: self.extra_time_ms,
            solution_name: sln.name.to_string(),
            entry: sln.entry,
        })
    }

    pub fn make_solution_items(&self) -> Vec<SolutionItem> {
        self.solutions.iter().enumerate().map(|(index, sln)| SolutionItem {
            id: self.id,
            index: index as i64,
            answer: self.answer,
            extra_time_ms: self.extra_time_ms,
            solution_name: sln.name.to_string(),
            entry: sln.entry,
         }).collect()
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
}

pub struct SolutionItem {
    pub id: i64,
    pub index: i64,
    pub answer: Option<i64>,
    pub extra_time_ms: u64,
    pub solution_name: String,
    pub entry: Solution,
}

impl SolutionItem {
    pub fn run_result(&self) -> RunResult {
        RunResult::init(self.solution_name.to_string(), self.answer, self.extra_time_ms)
    }

    pub fn finish_result(&self, got: i64, cost: time::Duration) -> RunResult {
        self.run_result().finish(got, cost)
    }

    pub fn timeout_result(&self, cost: time::Duration) -> RunResult {
        self.run_result().timeout(cost)
    }

    pub fn crash_result(&self, cost: time::Duration) -> RunResult {
        self.run_result().crash(cost)
    }
}
