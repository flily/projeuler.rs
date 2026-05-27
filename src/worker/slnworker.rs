use std::time;

use crate::common::{Problem};

use super::{RunResult, RunError};


pub struct Worker {
    pub problems: Vec<Problem>,
}

impl Worker {
    pub fn on(problems: Vec<Problem>) -> Self {
        Worker { problems: problems.to_vec() }
    }

    pub fn on_static(problems: Vec<&'static Problem>) -> Self {
        let ps = problems.iter().map(|p| (*p).clone()).collect();
        Worker::on(ps)
    }

    pub fn get_problem(&self, problem_id: i64) -> Result<Problem, RunError> {
        match self.problems.iter().find(|p| p.id == problem_id) {
            Some(p) => Ok(p.clone()),
            None => Err(RunError::ProblemNotFound { problem_id }),
        }
    }

    pub fn run(&self, pid: i64, sid: usize) -> Result<RunResult, RunError> {
        let problem = self.get_problem(pid)?;
        let solution = problem.get_solution(sid);
        if solution.is_none() {
            return Err(RunError::SolutionNotFound { problem_id: pid, solution_id: sid });
        }

        let solution = solution.unwrap();
        let start = time::Instant::now();
        let got = (solution.entry)();
        let cost = start.elapsed();

        let mut result = solution.finish_result(got, cost);
        result.check();

        Ok(result)
    }
}
