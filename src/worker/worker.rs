use std::time;

use crate::common::{Problem, Solution, SolutionInfo};

use super::{FinalResult, RunResult, RunError};


pub struct Worker {
    pub problems: Vec<Problem>,
}

impl Worker {
    pub fn on(problems: Vec<Problem>) -> Self {
        Worker { problems: problems.iter().map(|p| p.clone()).collect() }
    }

    pub fn on_static(problems: Vec<&'static Problem>) -> Self {
        let ps = problems.iter().map(|p| (*p).clone()).collect();
        Worker::on(ps)
    }

    pub fn get_solution(&self, problem_id: i64, solution_id: usize) -> Result<SolutionInfo, RunError> {
        let problem = self.problems.iter().find(|p| p.id == problem_id);
        if problem.is_none() {
            return Err(RunError::ProblemNotFound { problem_id });
        }

        let problem = problem.unwrap();
        if solution_id >= problem.solutions.len() {
            return Err(RunError::SolutionNotFound { problem_id, solution_id });
        }

        Ok(problem.solutions[solution_id].clone())
    }

    pub fn make_result(&self, problem_id: i64, solution_id: usize) -> Result<RunResult, RunError> {
        let problem = self.problems.iter().find(|p| p.id == problem_id);
        if problem.is_none() {
            return Err(RunError::ProblemNotFound { problem_id });
        }

        let problem = problem.unwrap();
        let result = problem.make_run_result_for(solution_id);
        if result.is_none() {
            return Err(RunError::SolutionNotFound { problem_id, solution_id });
        }

        Ok(RunResult {
            solution: problem.solutions[solution_id].name.to_string(),
            entry: problem.solutions[solution_id].entry,
            answer: Some(problem.answer),
            got: None,
            result: FinalResult::None,
            cost: time::Duration::from_secs(0),
            extra_timeout_ms: problem.extra_time_ms,
        })
    }
}
